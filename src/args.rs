use clap::Parser;
use std::path::PathBuf;

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

    /// Verbose
    #[arg(long, short, default_value_t = false)]
    pub verbose: bool,
}
