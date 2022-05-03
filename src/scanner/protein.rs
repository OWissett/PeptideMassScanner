// Copyright (c) 2022 Oliver Wissett
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// Cargo Package Imports
use std::{collections::HashMap, str::FromStr};

lazy_static! {
    pub static ref AMINO_ACIDS: HashMap<char, f64> = HashMap::from([
        ('A', 89.094),
        ('R', 174.203),
        ('N', 132.119),
        ('D', 133.104),
        ('C', 121.154),
        ('Q', 146.146),
        ('E', 147.131),
        ('G', 75.067),
        ('H', 155.156),
        ('I', 131.175),
        ('L', 131.175),
        ('K', 146.189),
        ('M', 149.208),
        ('F', 165.192),
        ('P', 115.132),
        ('S', 105.093),
        ('T', 119.119),
        ('W', 204.228),
        ('Y', 181.191),
        ('V', 117.148),
    ]);
}

#[derive(Debug)]
pub struct Protein {
    pub primary_seq: String,
    pub length: usize,
    pub mass: f64,
}

impl Protein {
    pub fn new(primary_seq: &str) -> Result<Protein, &'static str> {
        if !Protein::is_valid_sequence(&primary_seq) {
            return Err("Invalid protein sequence!");
        }
        Ok(Protein {
            primary_seq: String::from_str(primary_seq).unwrap(),
            length: primary_seq.chars().count(),
            mass: Protein::calculate_mass(primary_seq),
        })
    }

    // Static Methods
    fn is_valid_sequence(seq: &str) -> bool {
        if seq.len() == 0 {
            return false;
        };

        for (_, aa) in seq.chars().enumerate() {
            if !AMINO_ACIDS.contains_key(&aa) {
                return false;
            };
        }

        true
    }

    pub fn calculate_mass(pep_str: &str) -> f64 {
        let mut total_mass: f64 = 0.0;

        for (_, aa) in pep_str.chars().enumerate() {
            total_mass += AMINO_ACIDS[&aa] - 18.01528; // correct for condensation reaction mass loss
        }

        total_mass = total_mass + 18.01528; // N-terminal (+1.01 for extra proton [H+]) and C-terminal correction (+15.99 for extra oxygen)

        if cfg!(debug_assertion) {
            print!("Peptide total mass: {}\n", total_mass);
        }

        total_mass

        // Optimisation - exit early if mass is above the target mass... (potentially save lots of time)
    }
}


// TODO: Add Display trait to protein