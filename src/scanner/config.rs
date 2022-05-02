pub use clap::Parser;

// Config struct to store CLI arguments
#[derive(Parser)]
#[clap(
    author = "Oliver Wissett",
    version,
    about = "A tool to search for peptide masses within a protein sequence written in Rust."
)]
pub struct Config {
    pub target_mass: f64,

    #[clap(
        short = 'm',
        default_value = "1.0"
    )]
    pub mass_tolerance: f64,

    #[clap(
        parse(from_os_str), 
        forbid_empty_values = true
    )]
    pub seq_path: std::path::PathBuf,

    #[clap(
        short = 'w',
        default_value = "1"
    )]
    pub min_window_length: usize,
}
