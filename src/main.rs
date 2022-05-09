mod lib;

use clap::{Parser, Subcommand};
use lib::{decode as c_api_decode, encode as c_api_encode};
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode string to base64 string
    Encode { input: String },

    /// Decode base64 string to string
    Decode { input: String },
}

fn encode(input: &str) -> String {
    let input_c_str = CString::new(input).unwrap();
    let input_c_char: *const c_char = input_c_str.as_ptr() as *const c_char;
    let output_c_str = unsafe { CString::from_raw(c_api_encode(input_c_char)) };
    output_c_str.to_str().unwrap().to_string()
}
fn decode(input: &str) -> String {
    let input_c_str = CString::new(input).unwrap();
    let input_c_char: *const c_char = input_c_str.as_ptr() as *const c_char;
    let output_c_str = unsafe { CString::from_raw(c_api_decode(input_c_char)) };
    output_c_str.to_str().unwrap().to_string()
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encode { input } => {
            println!("{}", encode(input));
        }
        Commands::Decode { input } => {
            println!("{}", decode(input));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode("hello"), "aGVsbG8=");
    }
    #[test]
    fn test_decode() {
        assert_eq!(decode("aGVsbG8="), "hello");
    }
}
