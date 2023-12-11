use ju_tcs_rust_23_20::*;
use std::path::PathBuf;

use clap::Parser;

/// Simple program to get a
#[derive(Parser, Debug)]
enum Cmd {
    Head { n: usize, path: PathBuf },
    Tail { n: usize, path: PathBuf },
}
fn main() -> Result<(), &'static str> {
    let parse_result = Cmd::parse();
    let result = match &parse_result {
        Cmd::Head { n, path } => head(path, *n),
        Cmd::Tail { n, path } => tail(path, *n),
    };
    let lines = match result {
        Ok(lines) => lines,
        Err(_) => {
            return Err("Unable to read file!");
        }
    };
    for line in lines {
        println!("{line}");
    }
    Ok(())
}
