use rusqlite::{Connection, Error};

pub fn create_schema(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE metadata (
          name TEXT NOT NULL,
          value TEXT NOT NULL,
          UNIQUE(name)
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE tiles (
          zoom_level INTEGER NOT NULL,
          tile_column INTEGER NOT NULL,
          tile_row INTEGER NOT NULL,
          tile_data BLOB NOT NULL,
          PRIMARY KEY (zoom_level, tile_column, tile_row)
        )",
        (),
    )?;

    Ok(())
}
