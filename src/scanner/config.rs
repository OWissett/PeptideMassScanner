pub use clap::Parser;

// Config struct to store CLI arguments
#[derive(Parser)]
#[clap(
    author = "Oliver Wissett",
    version,
    about = "A tool to search for peptide masses within a protein sequence written in Rust."
)]
pub struct Config {
    pub search_mass: i32,

    #[clap(short = 'm')]
    pub mass_tolerance: Option<f64>,

    #[clap(parse(from_os_str))]
    pub seq_path: std::path::PathBuf,
}
