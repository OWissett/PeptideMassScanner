mod protein;
mod config;

pub use config::{Parser, Config};
use protein::*;

use std::{
    error::Error, 
    fs::read_to_string,
    collections::HashMap,
};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // run the scanner...
    let sequence_input = match read_to_string(config.seq_path) {
        Ok(seq) => seq,
        Err(err) => return Err(Box::new(err)),
    };

    let protein = Protein::new(&sequence_input)?;

    // No need to search if the target mass is greater than 
    if protein.mass + config.mass_tolerance > config.target_mass { 
        println!("Search mass exceeds the total mass of protein. No match possible.");
        return Ok(());
    }

    // TODO: implement protein scanning
    let mut positive_hits = HashMap::new();
    
    // dummy code to be removed, here to make compiler happy... (for now)
    positive_hits.insert("k", 1.0);

    for window_size in config.min_window_length..protein.length {
        
    }

    Ok(())
}

fn determine_min_window_size(mass: f64) -> usize {
    // TODO: implement...

    return 1;
}

fn is_within_tolerance(pep_mass: f64, target_mass: f64, tolerance: f64) -> bool {
    if pep_mass <= target_mass + tolerance && pep_mass >= target_mass - tolerance {
        return true;
    };
    false
}

#[cfg(test)]
mod tests {



}