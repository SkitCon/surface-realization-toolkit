///
/// File: lib.rs
/// Author: Amber Converse
/// Purpose: This file contains utility functions for main.rs and provides a framework for generating
///     a Python library wrapper for this code
///

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rustfst::prelude::*;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

/// Wrapper for Python module code to call realize_query
#[pyfunction]
fn realize_query_py(fst_path: &str, query: &str) -> PyResult<String> {
    realize_query(fst_path, query).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

/// Python module definition
#[pymodule]
fn fst_realize(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(realize_query_py, m)?)?;
    Ok(())
}

/// Generate an FST from an input file formatted as:
/// lemma: form1+CAT1+CAT2, form2+CAT1+CAT2
pub fn generate_fst(input_file: &str, output_fst_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut fst = VectorFst::new();
    let start_state = fst.add_state();
    fst.set_start(start_state)?;

    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(entry) = line {
                process_entry(&mut fst, &entry, start_state)?;
            }
        }
    }

    fst.write(output_fst_file)?;
    println!("FST saved to {}", output_fst_file);
    Ok(())
}

/// Realize a morphological form from an FST.
pub fn realize_query(fst_path: &str, query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let fst = VectorFst::read(fst_path)?;
    let tokenizer: Vec<&str> = query.split('+').collect();

    if tokenizer.is_empty() {
        return Err("Query is empty or malformed.".into());
    }

    let word = tokenizer[0];
    let tags = &tokenizer[1..];

    let input_symbols: Vec<_> = word.chars().chain(tags.iter().flat_map(|t| t.chars())).collect();

    let mut state = fst.start().ok_or("FST has no start state.")?;
    let mut output = String::new();

    for symbol in input_symbols {
        let label = symbol as u32;

        if let Some(arc) = fst.arcs(state)?.find(|arc| arc.ilabel == label) {
            output.push(char::from_u32(arc.olabel).unwrap_or_default());
            state = arc.nextstate;
        } else {
            return Err(format!("No valid path for symbol: {}", symbol).into());
        }
    }

    if fst.is_final(state) {
        Ok(output)
    } else {
        Err("No valid path to a final state.".into())
    }
}

/// Utility function to read lines from a file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Process a single entry from the input file.
fn process_entry(fst: &mut VectorFst, entry: &str, start_state: StateId) -> Result<(), Box<dyn std::error::Error>> {
    let mut parts = entry.split(":");
    let lemma = parts.next().ok_or("Missing lemma in entry")?.trim();
    let forms = parts.next().ok_or("Missing forms in entry")?.split(",");

    for form in forms {
        let form = form.trim();
        if let Some((word, tags)) = form.split_once('+') {
            add_transduction(fst, start_state, lemma, word, tags)?;
        }
    }

    Ok(())
}

/// Add a transduction to the FST.
fn add_transduction(fst: &mut VectorFst, start_state: StateId, lemma: &str, word: &str, tags: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut current_state = start_state;

    for c in lemma.chars().chain(tags.chars()).chain(word.chars()) {
        let next_state = fst.add_state();
        fst.add_arc(current_state, Arc::new(c as u32, c as u32, 0.0, next_state))?;
        current_state = next_state;
    }

    fst.set_final(current_state, 0.0)?;
    Ok(())
}
