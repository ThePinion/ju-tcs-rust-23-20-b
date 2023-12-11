use ju_tcs_rust_23_20::*;
use std::path::PathBuf;

use clap::Parser;

/// Simple program to get a
#[derive(Parser, Debug)]
enum Cmd {
    Head { n: usize, path: PathBuf },
    Tail { n: usize, path: PathBuf },
}
fn main() {
    let lines = match Cmd::parse() {
        Cmd::Head { n, path } => head(&path, n).unwrap(),
        Cmd::Tail { n, path } => tail(&path, n).unwrap(),
    };
    for line in lines {
        println!("{line}");
    }
}
