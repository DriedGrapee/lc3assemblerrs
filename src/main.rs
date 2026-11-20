/* Step 1: Read an assembly file (.asm)
 * Step 2: Create new output file
 * Step 3: Pass 1 copies lines over, determines their address and creates symbol table
 * Step 4: Pass 2 calculates offsets then converts to binary
 */

#![feature(file_buffered)]

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    input_path: std::path::PathBuf,
    output_path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let input_file = File::open(&args.input_path)
        .with_context(|| format!("could not read file `{}`", args.input_path.display()))?;
    let output_file = File::create_buffered(&args.output_path)
        .with_context(|| format!("could not create file `{}`", args.output_path.display()))?;

    let mut reader = BufReader::new(input_file);
    let mut line = String::new();

    let origin: u16;
    let offset_from_origin: u16;

    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            // return some sort of error with context about how there is no origin
        } else {
            let mut trimmed_line = line.trim();
            if &trimmed_line[..5] == ".ORIG" {
                trimmed_line = &trimmed_line[5..];
                let numeric_literal_base = (&trimmed_line)
                    .char_indices()
                    .find(|&(_, c)| !c.is_whitespace());
                trimmed_line = &trimmed_line[numeric_literal_base.unwrap().0 + 1..];
                let end = trimmed_line
                    .char_indices()
                    .find(|&(_, c)| !c.is_ascii_digit())
                    .map(|(idx, _)| idx)
                    .unwrap_or(trimmed_line.len());

                let string_number = &trimmed_line[..end];
                match numeric_literal_base.unwrap().1 {
                    'b' => {
                        origin = u16::from_str_radix(string_number, 2).unwrap();
                        break;
                    }
                    '#' => {
                        origin = u16::from_str_radix(string_number, 10).unwrap();
                        break;
                    }
                    'x' => {
                        origin = u16::from_str_radix(string_number, 16).unwrap();
                        break;
                    }
                    _ => {
                        // custom error because .orig was formed incorrectly. this will be some sort
                        // of assembler error that I use throughout
                    }
                }
            }
        }
    }

    while reader.read_line(&mut line)? != 0 {
        line.clear();
    }
    Ok(())
}
