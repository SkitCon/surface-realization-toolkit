///
/// File: generate_fst.rs
/// Author: Amber Converse
/// Purpose: To generate an FST for surface realization of morphology based on an input file in the format:
/// 
///         lemma: form1+CAT1+CAT2, form2+CAT1+CAT2
/// 
///     Usage:
///         ./generate_fst <input file> <output file>
/// 

use rustfst::prelude::*;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_fst_file>", args[0]);
        std::process::exit(1);
    }
    let input_file = &args[1];
    let output_fst_file = &args[2];

    // Create a mutable FST
    let mut fst = VectorFst::new();

    // Add the start state
    let start_state = fst.add_state();
    fst.set_start(start_state)?;

    // Read the input file line by line
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(entry) = line {
                process_entry(&mut fst, &entry, start_state)?;
            }
        }
    }

    // Save the FST to the output file
    fst.write(output_fst_file)?;

    println!("FST saved to {}", output_fst_file);
    Ok(())
}

// Process each line of the input file
fn process_entry(fst: &mut VectorFst, entry: &str, start_state: StateId) -> Result<(), Box<dyn std::error::Error>> {
    // Split the line into lemma and forms
    let mut parts = entry.split(":");
    let lemma = parts
        .next()
        .ok_or("Missing lemma in entry")?
        .trim();
    let forms = parts
        .next()
        .ok_or("Missing forms in entry")?
        .split(",");

    for form in forms {
        let form = form.trim();
        // Split each form into the word and its morphological tags
        if let Some((word, tags)) = form.split_once('+') {
            add_transduction(fst, start_state, lemma, word, tags)?;
        }
    }

    Ok(())
}

// Add a transduction to the FST
fn add_transduction(fst: &mut VectorFst, start_state: StateId, lemma: &str, word: &str, tags: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut current_state = start_state;

    // Add transitions for the lemma
    for c in lemma.chars() {
        let next_state = fst.add_state();
        fst.add_arc(current_state, Arc::new(c as u32, c as u32, 0.0, next_state))?;
        current_state = next_state;
    }

    // Add transitions for the morphological tags
    for c in tags.chars() {
        let next_state = fst.add_state();
        fst.add_arc(current_state, Arc::new(c as u32, c as u32, 0.0, next_state))?;
        current_state = next_state;
    }

    // Add the word output as the final transition
    let word_state = fst.add_state();
    for c in word.chars() {
        let next_state = fst.add_state();
        fst.add_arc(current_state, Arc::new(c as u32, c as u32, 0.0, next_state))?;
        current_state = next_state;
    }

    fst.set_final(current_state, 0.0)?;
    Ok(())
}

// Utility function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}