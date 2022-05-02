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
    primary_seq: String,
}

impl Protein {
    pub fn new(primary_seq: &str) -> Result<Protein, &'static str> {
        if !Protein::is_valid_sequence(&primary_seq) {
            return Err("");
        }
        Ok(Protein {
            primary_seq: String::from_str(primary_seq).unwrap(),
        })
    }

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
}
