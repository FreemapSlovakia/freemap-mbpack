mod args;
mod schema;

use args::Args;
use clap::Parser;
use rusqlite::Connection;
use schema::create_schema;
use std::{error::Error, fs::File, io::Read, process::ExitCode};
use walkdir::{DirEntry, WalkDir};

fn main() -> ExitCode {
    if let Err(e) = try_main() {
        eprintln!("{e}");

        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn try_main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let target_file = args.target_file.as_path();

    if target_file.exists() {
        return Err("Target file exists".into());
    }

    let conn = Connection::open(target_file).map_err(|e| format!("Error creating output: {e}"))?;

    conn.pragma_update(None, "synchronous", "OFF")?;

    create_schema(&conn).expect("error initializing schema");

    let mut min_zoom: Option<u8> = None;

    let mut max_zoom: Option<u8> = None;

    for entry in WalkDir::new(args.source_dir.as_path()) {
        let entry = entry.map_err(|e| format!("Error walking directory: {e}"))?;

        if entry.file_type().is_dir() {
            continue;
        }

        let path = entry.path();

        let Ok((ext, z, x, y)) = parse_path(&entry) else {
            eprintln!("unexpected file, skipping: {}", path.display());

            continue;
        };

        let mut file = File::open(path).map_err(|e| format!("Error walking directory: {e}"))?;

        let mut data = Vec::new();

        file.read_to_end(&mut data)
            .map_err(|e| format!("Error reading file: {e}"))?;

        conn.execute(
            "INSERT INTO tiles VALUES (?1, ?2, ?3, ?4)",
            (z, x, (1 << z) - 1 - y, data), // TODO tms
        )
        .map_err(|e| format!("Error inserting tile {z}/{x}/{y}.{ext}: {e}"))?;

        min_zoom = Some(min_zoom.map_or(z, |zoom| zoom.min(z)));

        max_zoom = Some(max_zoom.map_or(z, |zoom| zoom.max(z)));
    }

    insert_metadata(&conn, &args, min_zoom, max_zoom, "jpeg")
        .map_err(|e| format!("Error inserting metadata: {e}"))?; // TODO format

    Ok(())
}

fn insert_metadata(
    conn: &Connection,
    args: &Args,
    min_zoom: Option<u8>,
    max_zoom: Option<u8>,
    format: &str,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO metadata (name, value) VALUES ('name', ?1);",
        [args
            .name
            .clone()
            .or_else(|| {
                args.target_file
                    .as_path()
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
            })
            .unwrap_or_else(|| "noname".into())],
    )?;

    conn.execute(
        "INSERT INTO metadata (name, value) VALUES ('format', ?1);",
        [format],
    )?;

    if let Some(min_zoom) = min_zoom {
        conn.execute(
            "INSERT INTO metadata (name, value) VALUES ('minzoom', ?1);",
            [min_zoom],
        )?;
    }

    if let Some(max_zoom) = max_zoom {
        conn.execute(
            "INSERT INTO metadata (name, value) VALUES ('maxzoom', ?1);",
            [max_zoom],
        )?;
    }

    Ok(())
}

fn parse_path(entry: &DirEntry) -> Result<(String, u8, u32, u32), ()> {
    let v = entry
        .path()
        .iter()
        .rev()
        .take(entry.depth())
        .map(|part| part.to_string_lossy().to_string())
        .collect::<Vec<_>>();

    if v.len() != 3 {
        return Err(());
    }

    let parts: Vec<_> = v.get(0).unwrap().splitn(2, '.').collect();

    if parts.len() != 2 {
        return Err(());
    }

    let y: u32 = parts.get(0).unwrap().parse().map_err(|_| {})?;
    let x: u32 = v.get(1).unwrap().parse().map_err(|_| {})?;
    let z: u8 = v.get(2).unwrap().parse().map_err(|_| {})?;

    Ok(((*parts.get(1).unwrap()).to_string(), z, x, y))
}
