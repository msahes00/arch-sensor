//! Arch-sensor

use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::env;
use std::fs;

/// Store the architecture name and the list of instructions
struct Arch {
    name: &'static str,
    list: &'static str,
}

// Ease the creation of Arch structs
macro_rules! arch {
    ($name:expr) => {
        Arch {
            name: $name,
            list: include_str!(concat!("resources/", $name, ".list")),
        }
    };
}

fn read_contents() -> Result<String, Box<dyn Error>> {

    // Get the arguments passed to the program
    let args: Vec<String> = env::args().collect();

    // Check if there are arguments
    if args.len() < 2 {
        return Err("No arguments provided.".into());
    }

    // If the first argument is "-", treat the rest as the content
    if args[1] == "-" {
        // Join the rest of the arguments after the dash
        let content = args[2..].join(" ");
        
        // Return the contents as text
        return Ok(content);

    } else {
        // Try to open the file specified by the first argument
        let file_content = fs::read_to_string(&args[1])?;

        // If the file is read successfully, return its contents
        return Ok(file_content);
    }
}

fn parse_mnemonics(contents: String) -> Result<HashSet<String>, Box<dyn Error>> {

    let mut words = HashSet::new();

    // Split the contents by anything but letters or numbers (bruteforce approach)
    for token in contents.split(|c: char| !c.is_alphanumeric()) {
        if !token.is_empty() {
            words.insert(token.to_string());
        }
    }

    return Ok(words);
}

fn detect_architectures(mnemonics: HashSet<String>, architectures: Vec<Arch>) -> Result<HashMap<String, i64>, Box<dyn Error>> {

    let mut counts = HashMap::new();

    for arch in architectures {

        // Split the 'list' field into mnemonics and collect into a HashSet
        let instructions: HashSet<String> = arch
            .list
            .split('\n')
            .map(|s| s.trim().to_string()) // Trim spaces and convert to String
            .collect();

        // Count matches with the input mnemonics
        let matches = mnemonics.intersection(&instructions).count();

        // Insert the mnemonic count into the map
        counts.insert(arch.name.to_string(), matches as i64);
    }

    return Ok(counts);
}

fn print_results(results: HashMap<String, i64>) -> Result<(), Box<dyn Error>> {

    // Check if the results map is empty
    if results.is_empty() {
        println!("No results to display.");
        return Ok(());
    }

    println!("Detected Architecture Mnemonic Counts:");
    println!("--------------------------------------");

    // Print each architecture's result
    for (arch_name, count) in &results {
        println!("{}: {}", arch_name, count);
    }

    println!("--------------------------------------");

    // Find the architecture(s) with the maximum match count
    let max_count = results.values().cloned().max().unwrap_or(0);
    let top_architectures: Vec<_> = results
        .iter()
        .filter(|(_, &count)| count == max_count)
        .map(|(arch_name, _)| arch_name.clone())
        .collect();

    // Print a verdict
    if max_count == 0 {
        println!("No known architecture found.");

    } else if top_architectures.len() == 1 {

        println!("The detected architecture is '{}' (most likely).", top_architectures[0]);
    } else {

        println!("Unable to detect the architecture due to ambiguity");
    }
    Ok(())
}



fn main() -> Result<(), Box<dyn Error>>{
    // Store all the available architectures
    let architectures = vec![
        arch!("arm"),
        arch!("avr"),
        arch!("intel"),
        arch!("moxie"),
        arch!("powerpc"),
        arch!("riscv"),
        arch!("sparc"),
        arch!("w6502"),
    ];

    // Read the specified file or from stdin
    let contents = read_contents()?;

    // Filter mnemonics
    let mnemonics = parse_mnemonics(contents)?;

    // Find the architecture of the mnemonics
    let results = detect_architectures(mnemonics, architectures)?;

    // Print the results
    print_results(results)?;

    // End without any problems
    return Ok(());
}
