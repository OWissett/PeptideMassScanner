// Copyright (c) 2022 oliver
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod app_error;
mod config;
mod protein;

use app_error::AppError;
use protein::*;
pub use config::{Config, Parser};

use std::{collections::HashMap, error::Error, fs::read_to_string};

type SeqMap = HashMap<String, (f64, i32)>; // HashMap<Protein Sequence, (fragment mass, number of occurrences)>

struct Scanner {
    protein: Protein,           // The protein to be scanned
    target_mass: f64,           // Target mass of fragment
    mass_tolerance: f64,        // The amount of wiggle room around the mass that is acceptable
    positive_hits: Box<SeqMap>, // Stores all of the possible peptides fragments sequences (as keys), and a tuple of mass and number of occurrences
}

impl Scanner {
    // Constructor
    pub fn new(
        protein: Protein,
        target_mass: f64,
        mass_tolerance: f64,
    ) -> Result<Scanner, Box<dyn Error>> {
        // No need to search if the target mass is greater than total mass
        if target_mass - mass_tolerance > protein.mass {
            return Err(Box::new(AppError {
                msg: "Search mass exceeds the total mass of protein. No match possible."
                    .to_string(),
            }));
        }

        if target_mass + mass_tolerance < AMINO_ACIDS[&'G'] {
            return Err(Box::new(AppError {
                msg: "Search mass is lower than a single glycine residue. No match possible."
                    .to_string(),
            }));
        }

        let scanner = Scanner {
            protein,
            target_mass,
            mass_tolerance,
            positive_hits: Box::new(SeqMap::new()),
        };

        return Ok(scanner);
    }

    // Member Functions

    pub fn scan(&mut self) -> Vec<(&String, &(f64, i32))> {
        let frag_mat = self.calculate_frags_and_search();
        
        println!(
            "Input Sequence:\n{}\n\nTarget mass: {}\nMass tolerance: {}",
            self.protein.primary_seq,
            self.target_mass,
            self.mass_tolerance
        );

        println!("Fragment Matrix:");

        // TODO: Add switch in config for displaying frag matrix
        
        for row in frag_mat {
            println!("{:?}", &row);
        }

        
        println!("Hits: {:?}", self.positive_hits.keys());

        let hits = Vec::from_iter(self.positive_hits.iter());

        return hits;
    }

    // This function may be modified to be run in parallel - each row can be computed independently
    fn calculate_frags_and_search(&mut self) -> Vec<Vec<f64>> {
        // Time complexity of matrix calculation: O(n^2/2 + n)

        let n = self.protein.primary_seq.len();
        let seq_bytes = self.protein.primary_seq.as_bytes();
        let mut f = vec![vec![0.0; n]; n];

        for row in 0..n {
            for fragment in row..n {

                if row == fragment {

                    f[row][fragment] += AMINO_ACIDS[&(seq_bytes[fragment] as char)];

                    if self.in_tolerance(f[row][fragment]) {
                        let frag_seq = self.protein.primary_seq[row..fragment + 1].to_string();
                        match self.positive_hits.entry(frag_seq) {
                            std::collections::hash_map::Entry::Occupied(mut o) => {
                                o.get_mut().1 += 1;
                            }
                            std::collections::hash_map::Entry::Vacant(v) => {
                                v.insert((f[row][fragment], 1));
                            }
                        }
                    };
                    continue;
                }


                f[row][fragment] =
                    f[row][fragment - 1] + AMINO_ACIDS[&(seq_bytes[fragment] as char)] - 18.01528;


                if self.in_tolerance(f[row][fragment]) {
                    let frag_seq = self.protein.primary_seq[row..fragment + 1].to_string();
                    match self.positive_hits.entry(frag_seq) {
                        std::collections::hash_map::Entry::Occupied(mut o) => {
                            o.get_mut().1 += 1;
                        }
                        std::collections::hash_map::Entry::Vacant(v) => {
                            v.insert((f[row][fragment], 1));
                        }
                    }
                };

            }
        }
        return f;
    }

    fn in_tolerance(&self, frag_mass: f64) -> bool {
        (frag_mass <= self.target_mass + self.mass_tolerance)
            && (frag_mass >= self.target_mass - self.mass_tolerance)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // run the scanner...
    let sequence_input = match read_to_string(config.seq_path) {
        Ok(seq) => seq,
        Err(err) => return Err(Box::new(err)),
    };

    let mut scanner = Scanner::new(
        Protein::new(&sequence_input)?,
        config.target_mass,
        config.mass_tolerance,
    )?;

    scanner.scan();

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::{Scanner, Protein};




    #[test]
    fn mock_up()  {
        let sequence_input = "GAKAATGY";
        let target_mass = 274.304;
        let mass_tolerance = 1.0;

        let scanner = Scanner::new(
            Protein::new(&sequence_input).unwrap(),
            target_mass,
            mass_tolerance,
        );
        
        match scanner {
            Ok(mut scanner) => {
                let res = scanner.scan();
                println!("{:?}", res);
                assert_eq!(res[0].0, "GAK")
            },
            Err(e) => { 
                eprintln!("{}", e);
                panic!();
            },
        }
        
    }
    
}
