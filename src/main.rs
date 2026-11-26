/* Step 1: Read an assembly file (.asm)
 * Step 2: Create new output file
 * Step 3: Pass 1 copies lines over, determines their address and creates symbol table
 * Step 4: Pass 2 calculates offsets then converts to binary
 */

#![allow(unused_variables)]
#![allow(unused_imports)]

mod util;

use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use anyhow::{Context, Result, bail};
use clap::Parser;

static OPCODES: &[&str] = &[
    "ADD", "AND", "NOT", "BR", "JMP", "JSR", "JSRR", "RET", "LD", "LDI", "LDR", "LEA", "ST", "STI",
    "STR", "TRAP", "RTI",
];

#[derive(Parser)]
struct Cli {
    input_path: std::path::PathBuf,
    output_path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut reader = util::open_buffered(&args.input_path)
        .with_context(|| format!("could not read file `{}`", args.input_path.display()))?;
    let mut output_file = util::create_buffered(&args.output_path)
        .with_context(|| format!("could not create file `{}`", args.output_path.display()))?;

    let mut line = String::new();

    let origin: u16;
    let mut offset_from_origin: u16 = 0;

    let len = std::fs::metadata(&args.input_path)?.len() as usize;
    let capacity = len / 20; // rough guess at bytes per line

    let mut symbol_table: FxHashMap<String, u16> =
        FxHashMap::with_capacity_and_hasher(capacity, Default::default());

    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            bail!("No Origin Found. Consider adding .ORIG somewhere in your file.");
        } else {
            let mut trimmed_line = line.trim();
            if trimmed_line.is_empty() || trimmed_line == "\n" || &trimmed_line[0..1] == ";" {
                continue;
            }
            if trimmed_line.len() >= 5 && &trimmed_line[..5] == ".ORIG" {
                trimmed_line = &trimmed_line[5..];
                let numeric_literal_base = trimmed_line
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
                        bail!("No Origin Found. Consider adding .ORIG somewhere in your file.");
                    }
                }
            }
        }
    }
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            break; // EOF has been reached
        } else {
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() || trimmed_line == "\n" || &trimmed_line[0..1] == ";" {
                continue;
            }
            let instruction: Vec<&str> = trimmed_line
                .split(';')
                .next()
                .unwrap()
                .split(|c: char| c.is_whitespace() || c == ',')
                .filter(|s| !s.is_empty())
                .collect();

            let (opcode_original_case, arguments) =
                instruction.split_first().context("Empty Line Somehow!")?;

            let opcode = opcode_original_case.to_uppercase();
            // match all of the opcodes
            match (opcode.as_str(), arguments) {
                ("ADD" | "AND", [dr, sr1, sr2_or_immediate]) => {

                    //handle each opcode individually
                    // use .get.ok_or() and return custom error
                }
                ("NOT", [dr, sr]) => {}
                ("BR", [pc_offset_9]) => {} // should match BR, BRn, BRnz, BRnzp, etc.
                ("JMP" | "JSRR", [base_r]) => {}
                ("JSR", [pc_offset_11]) => {}
                ("RET" | "RTI", []) => {} // this is just JMP R7
                ("LD", [dr, pc_offset_9]) => {}
                ("LDI", [dr, pc_offset_9]) => {}
                ("LDR", [dr, br, offset_6]) => {}
                ("LEA", [dr, pc_offset_9]) => {}
                ("ST", [sr, pc_offset_9]) => {}
                ("STI", [sr, pc_offset_9]) => {}
                ("STR", [sr, br, offset_6]) => {}
                ("TRAP", [trap_vec_8]) => {}

                (op, _) if OPCODES.contains(&op) || is_branch(op) => {
                    bail!("{op}: wrong number of operands (got {})", arguments.len());
                }

                // Anything else is a label, possibly followed by an instruction
                (_, rest) => {
                    symbol_table.insert(
                        opcode_original_case.to_string(),
                        origin + offset_from_origin,
                    );
                    if !rest.is_empty() {
                        // e.g. "LOOP ADD R1, R1, #-1": run `rest` through this same match
                    }
                }
            }

            /*
            println!("{} {:?}", origin + offset_from_origin, instruction);
            offset_from_origin += 1;
            */
        }
    }
    while reader.read_line(&mut line)? != 0 {
        line.clear();
    }

    output_file.flush()?;
    Ok(())
}

fn is_branch(opcode: &str) -> bool {
    true
}
