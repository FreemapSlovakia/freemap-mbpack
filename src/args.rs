use clap::Parser;
use std::path::PathBuf;

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum Scheme {
    XYZ,
    TMS,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input directory
    pub source_dir: PathBuf,

    /// Output *.mbtiles file
    pub target_file: PathBuf,

    /// Name
    #[arg(long, short)]
    pub name: Option<String>,

    /// Tile scheme in the directory
    #[arg(long, short, value_enum, default_value_t = Scheme::XYZ)]
    pub scheme: Scheme,

    /// Verbose
    #[arg(long, short, default_value_t = false)]
    pub verbose: bool,
}
