// src/main.rs

mod delta_structs;
mod algo;

use delta_structs::{DeltaH, DeltaV};
use algo::{calculate_new_dh_values, calculate_new_dv_values, generate_match_vectors};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

// Function to read a FASTA file and return the sequences.
fn read_fasta(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut sequences = Vec::new();
    let mut current_sequence = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('>') {
            if !current_sequence.is_empty() {
                sequences.push(current_sequence.clone());
                current_sequence.clear();
            }
        } else {
            current_sequence.push_str(line.trim());
        }
    }
    // Push the last sequence
    if !current_sequence.is_empty() {
        sequences.push(current_sequence);
    }

    Ok(sequences)
}


fn main() -> io::Result<()> {
    // --- Read sequences from FASTA file ---
    let sequences = read_fasta("sequences.fasta")?;
    if sequences.len() < 2 {
        eprintln!("Error: FASTA file must contain at least two sequences.");
        std::process::exit(1); // Exit with an error code
    }
    let seq1 = &sequences[0];
    let seq2 = &sequences[1];

    // --- Rest of your alignment logic ---

    let seq1_len = seq1.len();
    let seq2_len = seq2.len();

    let mut dh = DeltaH::new(seq1_len);
    let mut dv = DeltaV::new();

    let matches_vectors = generate_match_vectors(seq1);

    for (char_index, current_char) in seq2.chars().enumerate() {
        println!("**************************************************");
        println!("*  Processing character '{}' (index {}) from seq2  *", current_char, char_index);
        println!("**************************************************");

        let match_index = match current_char {
            'A' => 0,
            'C' => 1,
            'G' => 2,
            'T' => 3,
            _ => panic!("Invalid character"),
        };

        calculate_new_dv_values(seq1_len, &matches_vectors, match_index, &dh, &mut dv);
        calculate_new_dh_values(seq1_len, &matches_vectors, match_index, &mut dh, &dv);

        println!("DeltaH Values:");
        for i in -3..=4 {
            println!("  dH[{}]: {:064b}", i, dh.get(i)); // Corrected to 64 bits
        }

        println!("DeltaV Values:");
        for i in -3..=4 {
            println!("  dV[{}]: {:064b}", i, dv.get(i)); // Corrected to 64 bits
        }
        println!();
    }
    Ok(())
}