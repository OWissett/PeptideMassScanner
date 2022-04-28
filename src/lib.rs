///
/// MIT License
/// 
/// Copyright (c) 2022 Oliver Wissett
/// 
/// Permission is hereby granted, free of charge, to any person obtaining a copy
/// of this software and associated documentation files (the "Software"), to deal
/// in the Software without restriction, including without limitation the rights
/// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
/// copies of the Software, and to permit persons to whom the Software is
/// furnished to do so, subject to the following conditions:
/// 
/// The above copyright notice and this permission notice shall be included in all
/// copies or substantial portions of the Software.
/// 
/// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
/// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
/// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
/// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
/// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
/// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
/// SOFTWARE.
///

// Standard Library Imports
use std::collections::HashMap;

// Cargo Package Imports
use clap::Parser;

// Config struct to store CLI arguments
#[derive(Parser)]
#[clap(author="Oliver Wissett", version, about="A tool to search for peptide masses within a protein sequence written in Rust.")]
pub struct Config {
    search_mass: i32,

    #[clap(short = 'm')]
    mass_tolerance: Option<i32>,

    #[clap(parse(from_os_str))]
    seq_path: std::path::PathBuf,
}

impl Config {
    // Add amino acid string validation
}


// Functions

pub fn run(config: Config) {
    // run the scanner...

}

fn is_within_tolerance(pep_mass: f64, target_mass: f64, tolerance: f64) -> bool {
    if pep_mass <= target_mass + tolerance && 
       pep_mass >= target_mass - tolerance { return true };
    false
}

fn calculate_mass(pep_str: &str) -> f64 {
    let amino_acids = HashMap::from([
        ('A',89.094),
        ('R',174.203),
        ('N',132.119),
        ('D',133.104),
        ('C',121.154),
        ('Q',146.146),
        ('E',147.131),
        ('G',75.067),
        ('H',155.156),
        ('I',131.175),
        ('L',131.175),
        ('K',146.189),
        ('M',149.208),
        ('F',165.192),
        ('P',115.132),
        ('S',105.093),
        ('T',119.119),
        ('W',204.228),
        ('Y',181.191),
        ('V',117.148),
    ]);
    
    let mut total_mass: f64 = 0.0;

    for(_, aa) in pep_str.chars().enumerate() {
        total_mass += amino_acids[&aa] - 18.01528; // correct for condensation reaction mass loss
    }

    total_mass = total_mass + 18.01528; // N-terminal (+1.01 for extra proton [H+]) and C-terminal correction (+15.99 for extra oxygen)

    print!("{}\n", total_mass);

    total_mass 
}

#[cfg(test)]
mod tests {
    use super::*; // import symbols from enclosing namespace

    #[test]
    fn tolerance_test() {
        assert!(is_within_tolerance(10.0, 10.5, 0.5));
    }

    #[test]
    fn six_alanines() {
        assert!(
            is_within_tolerance(
                calculate_mass("AAAAAA"),
                444.49,
                0.1)
            );
    }

    #[test]
    fn beefy_peptide() {
        assert!(
            is_within_tolerance(
                calculate_mass("MKWVTFISLLLLFSSAYSRGVFRRDTHKSEIAHRFKDLGEEHFKGLVLIAFSQYLQQCPFDEHVKLVNELTEFAKTCVADESHAGCEKSLHTLFGDELCKVASLRETYGDMADCCEKQEPERNECFLSHKDDSPDLPKLKPDPNTLCDEFKADEKKFWGKYLYEIARRHPYFYAPELLYYANKYNGVFQECCQAEDKGACLLPKIETMREKVLASSARQRLRCASIQKFGERALKAWSVARLSQKFPKAEFVEVTKLVTDLTKVHKECCHGDLLECADDRADLAKYICDNQDTISSKLKECCDKPLLEKSHCIAEVEKDAIPENLPPLTADFAEDKDVCKNYQEAKDAFLGSFLYEYSRRHPEYAVSVLLRLAKEYEATLEECCAKDDPHACYSTVFDKLKHLVDEPQNLIKQNCDQFEKLGEYGFQNALIVRYTRKVPQVSTPTLVEVSRSLGKVGTRCCTKPESERMPCTEDYLSLILNRLCVLHEKTPVSEKVTKCCTESLVNRRPCFSALTPDETYVPKAFDEKLFTFHADICTLPDTEKQIKKQTALVELLKHKPKATEEQLKTVMENFVAFVDKCCAADDKEACFAVEGPKLVVSTQTALA"),
                69293.41,
                0.1)
            );
    }

}