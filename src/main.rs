use clap::Parser;
use std::error::Error;
use std::fs::read_to_string;

mod analysis;
mod interactive;
mod app;

/// Break One-Time Pad Encryption with key reuse
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File containing hexadecimal ciphertexts, delimited by new lines
    #[arg(short, long)]
    file: String,

    /// Name of the person to greet
    #[arg(short, long, default_value_t = String::from("result.json"))]
    output: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut ciphertexts = Vec::new();

    for line in read_to_string(args.file).unwrap().lines() {
        ciphertexts.push(hex_string_to_bytes(line)?);
    }

    let partial_key = analysis::recover_key(ciphertexts.clone());

    interactive::run(ciphertexts, partial_key, args.output)?;

    Ok(())
}
#[derive(Debug)]
enum HexError {
    InvalidHexadecimal,
}

// Implement the std::error::Error trait for the custom error type
impl Error for HexError {}

// Implement the std::fmt::Display trait to show a user-friendly error message
impl std::fmt::Display for HexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HexError::InvalidHexadecimal => write!(f, "Invalid hexadecimal string"),
        }
    }
}

fn hex_string_to_bytes(hex_str: &str) -> Result<Vec<u8>, HexError> {
    // Ensure the input string has an even length
    if hex_str.len() % 2 != 0 {
        return Err(HexError::InvalidHexadecimal);
    }

    // Parse the string in pairs and convert them to u8 values
    let mut bytes = Vec::new();
    for i in (0..hex_str.len()).step_by(2) {
        let hex_pair = &hex_str[i..i + 2];
        if let Ok(byte) = u8::from_str_radix(hex_pair, 16) {
            bytes.push(byte);
        } else {
            // Return the custom error if the parsing fails
            return Err(HexError::InvalidHexadecimal);
        }
    }

    Ok(bytes)
}
