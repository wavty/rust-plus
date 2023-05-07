use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::{BufRead, BufReader};

fn read_file_contents(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_lines(filename: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_contents() {
        let filename = "Cargo.toml";
        match read_file_contents(filename) {
            Ok(contents) => println!("Contents of {}:\n{}", filename, contents),
            Err(error) => println!("Failed to read {}: {}", filename, error),
        }
    }

    #[test]
    fn test_for_env_vars() {
        for arg in std::env::args() {
            println!("{}", arg);
        }
    }
}
