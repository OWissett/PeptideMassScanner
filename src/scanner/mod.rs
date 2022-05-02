mod protein;
mod config;

pub use config::{Parser, Config};
use protein::*;

use std::{error::Error, fs::read_to_string};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // run the scanner...
    let sequence_input = match read_to_string(config.seq_path) {
        Ok(seq) => seq,
        Err(err) => return Err(Box::new(err)),
    };

    let protein = Protein::new(&sequence_input)?;

    // TODO: implement protein scanning

    Ok(())
}

fn is_within_tolerance(pep_mass: f64, target_mass: f64, tolerance: f64) -> bool {
    if pep_mass <= target_mass + tolerance && pep_mass >= target_mass - tolerance {
        return true;
    };
    false
}

fn calculate_mass(pep_str: &str) -> f64 {
    let mut total_mass: f64 = 0.0;

    for (_, aa) in pep_str.chars().enumerate() {
        total_mass += AMINO_ACIDS[&aa] - 18.01528; // correct for condensation reaction mass loss
    }

    total_mass = total_mass + 18.01528; // N-terminal (+1.01 for extra proton [H+]) and C-terminal correction (+15.99 for extra oxygen)

    print!("{}\n", total_mass);

    total_mass
}

#[cfg(test)]
mod tests {



}