// Copyright (c) 2022 Oliver Wissett
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub use clap::Parser;

// Config struct to store CLI arguments
#[derive(Parser)]
#[clap(
    author = "Oliver Wissett",
    version,
    about = "A tool to search for peptide masses within a protein sequence written in Rust."
)]
pub struct Config {
    /// The mass of the fragment to search for.
    pub target_mass: f64,

    /// Allows for matching of masses within +/- of this value. Decreasing this value will result in a more stringent search.
    #[clap(
        short = 'm',
        default_value = "0.5"
    )]
    pub mass_tolerance: f64,

    /// Path to the file containing the protein sequence.
    #[clap(
        parse(from_os_str), 
        forbid_empty_values = true
    )]
    pub seq_path: std::path::PathBuf,

    /// Save the peptide fragment matrix. Recommend if repeat searches are to be made on long sequences.
    #[clap(
        short = 'f',
    )]
    pub save_frag_mat: bool,
}
