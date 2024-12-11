///
/// File: main.rs
/// Author: Amber Converse
/// Purpose: To define a program which either 1) realizes morphology of a lemma or 2) changes morphology
///     of an existing word.
///
///     This program expects either an existing morph.fst file or an input file called morph.txt in the
///     form:
/// 
///         lemma1: form1+CAT1+CAT2, form2+CAT1+CAT2
///         lemma2: form1+CAT1
///         ...
///         lemma3: form1+CAT1, form2+CAT1+CAT2, ... formN+CAT1
/// 
///     Usage:
///         ./srt_realize WORD+CAT1+CAT2+...+CATN
///     
///     Example:
///         ./srt_realize estar+PLU+IND+PRES+1P
///         output: estamos
/// 

use std::env;
use std::fs;
use morph_lib::generate_fst;
use morph_lib::realize_query;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} WORD+CAT1+CAT2", args[0]);
        std::process::exit(1);
    }

    let query = &args[1];
    let fst_path = "morph.fst";
    let input_file = "morph.txt";

    // Check if the FST exists; generate if not.
    if !fs::metadata(fst_path).is_ok() {
        println!("FST file not found. Generating from input file...");
        generate_fst(input_file, fst_path)?;
    }

    // Realize the query.
    match realize_query(fst_path, query) {
        Ok(output) => println!("Output: {}", output),
        Err(err) => eprintln!("Error: {}", err),
    }

    Ok(())
}