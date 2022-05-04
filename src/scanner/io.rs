// Copyright (c) 2022 Oliver Wissett
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::types::{Matrix2D, SeqMap};
use super::Protein;

use bio::io::fasta::Reader as FastaReader;
use std::error::Error;

use super::app_error::AppError;

#[derive(Debug)]
pub enum OutFileFormat {
    JSON,
    CSV,
}

pub mod writers {
    use super::*;

    pub fn write_matrix(data: Matrix2D, file_format: OutFileFormat) -> Result<(), Box<dyn Error>> {
        // TODO Implement fragment_matrix writer.

        // Allow saving as json or CSV

        todo!();
    }

    pub fn write_seq_map(data: SeqMap, file_format: OutFileFormat) -> Result<(), Box<dyn Error>> {
        // TODO Implement seq_map writer

        todo!();
    }
}

pub mod readers {


    use super::*;

    pub fn read_seq_file(seq_path: &str) -> Result<Vec<Protein>, Box<dyn Error>> {
        // Check the file extension and call the appropriate function, then return protein vector
        
        if let Some(_) = str::strip_suffix(seq_path, ".fasta") {
            return read_fasta(seq_path)
        }

        if let Some(_) = str::strip_suffix(seq_path, ".csv") {
            return read_csv(seq_path)
        }
        
        Err(Box::new(AppError { 
            msg: "Could not identify file extension. Please ensure the input file is correctly named (*.fasta or *.csv)!".to_string()
        }))
    }

    fn read_fasta(fasta_path: &str) -> Result<Vec<Protein>, Box<dyn Error>> {
        let records = FastaReader::from_file(fasta_path)?.records();
        let mut proteins = vec![];

        for record in records {
            let record = record?; // Unwrap the record
            proteins.push(Protein::new(
                record.id(),
                std::str::from_utf8(record.seq())?,
            )?)
        }

        Ok(proteins)
    }

    fn read_csv(csv_path: &str) -> Result<Vec<Protein>, Box<dyn Error>> {
        todo!();
    }

    pub fn read_frag_mat(frag_mat_path: &str) -> Result<Matrix2D, Box<dyn Error>> {
        // TODO Implement frag_mat reader
        todo!();
    }

    #[cfg(test)]
    mod reader_tests {

        use super::*;

        #[test]
        fn read_valid_fasta_single_record() {
            let records = read_fasta("./tests/data/MMP-26.fasta").unwrap();

            assert_eq!(records.len(), 1); // Only a single record should be present in the file

            for record in records {
                assert_eq!(&record.id, "MMP-26");
                assert_eq!(record.primary_seq.len(), 261);
            }
        }

        #[test]
        fn read_valid_fasta_multiple_records() {
            let records = read_fasta("./tests/data/multiple_sequences.fasta").unwrap();

            assert_eq!(records.len(), 6); // Only a single record should be present in the file

            for record in records {
                assert_eq!(&record.id, "MMP-26");
                assert_eq!(record.primary_seq.len(), 261);
            }
        }

        #[test]
        fn read_invalid_path() {
            let records = read_fasta("not_a_real_path");
            match records {
                Ok(_) => panic!(),
                Err(_) => (),
            }
        }

        #[test]
        fn read_invalid_sequence() {
            let records = read_fasta("./tests/data/invalid.fasta");
            match records {
                Ok(_) => panic!(),
                Err(_) => (),
            }
        }
    }
}
