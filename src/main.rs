#![allow(unused)]

mod reader;
mod printer;
mod mal_types;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::mal_types::{MalResult,MalErr};
use crate::reader::read_str;
use crate::printer::pr_str;

fn read(s: String) -> MalResult {
    read_str(s)
}
fn eval(s: MalResult) -> MalResult {s}
fn print(s: MalResult) -> String {
    match s {
        Ok(ok) => pr_str(ok),
        Err(e) => format!("{}", e.0),
    }
    
}

fn rep(s: String) -> String {
    print(eval(read(s)))
}


fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                println!("Line: {}", rep(line));
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    Ok(())
}
