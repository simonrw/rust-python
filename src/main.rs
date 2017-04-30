#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Parse(::std::num::ParseIntError);
        }
    }
}

use std::fs::File;
use std::io::prelude::*;
use errors::*;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    LoadConst(usize),
    StoreFast(usize),
    LoadFast(usize),
    BinaryAdd,
    ReturnValue,
}

#[derive(Debug, PartialEq, Eq)]
enum PythonType {
    Int(i64),
}

#[derive(Debug)]
struct Interpreter {
    stack: Vec<PythonType>,
    instruction_list: Vec<Instruction>,
}

/// Helper function to get instructions from a disassembly file
fn parse_test_instruction_list(filename: &str) -> Result<Vec<Instruction>> {
    let mut fs = File::open(filename)?;
    let mut contents = String::new();
    fs.read_to_string(&mut contents)?;
    let mut out = Vec::new();

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }

        let mut words = line.split_whitespace();
        let instruction = words.next().unwrap();

        match instruction {
            "STORE_FAST" => {
                let address: usize = words.next().unwrap().parse()?;
                out.push(Instruction::StoreFast(address));
            }
            "LOAD_FAST" => {
                let address: usize = words.next().unwrap().parse()?;
                out.push(Instruction::LoadFast(address));
            }
            "LOAD_CONST" => {
                let address: usize = words.next().unwrap().parse()?;
                out.push(Instruction::LoadConst(address));
            }
            "BINARY_ADD" => out.push(Instruction::BinaryAdd),
            "RETURN_VALUE" => out.push(Instruction::ReturnValue),
            _ => panic!("UNHANDLED DISASSEMBLY INSTRUCTION: `{}`", instruction),
        }
    }

    Ok(out)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_to_instruction_list() {
        let filename = "dis-examples/foo.dis";
        let instructions = parse_test_instruction_list(filename).unwrap();
        let expected = vec![Instruction::LoadConst(1),
                            Instruction::StoreFast(0),
                            Instruction::LoadFast(0),
                            Instruction::LoadConst(2),
                            Instruction::BinaryAdd,
                            Instruction::StoreFast(1),
                            Instruction::LoadFast(1),
                            Instruction::ReturnValue];
        assert_eq!(instructions, expected);
    }
}
