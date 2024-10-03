#![allow(unused)]

mod mal_env;
mod mal_types;
mod printer;
mod reader;

use std::{env, fs};

use mal_types::MalType;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::mal_types::{MalError, MalResult};
use crate::printer::pr_str;
use crate::reader::read_str;

fn main() {
    let mut args = env::args();
    let program = args.next().expect("ok");
    if let Some(filepath) = args.next() {
        match fs::read_to_string(&filepath) {
            Ok(data) => match run(format!("(module {} )", data)) {
                Ok(out) => println!("{}", out),
                Err(e) => eprintln!("Error: {:?}", e),
            },
            Err(err) => eprintln!("Error: Cannot read file `{filepath}`.\n{err}"),
        }
    } else {
        usage(program);
    }
}

fn usage(program: String) {
    println!("Usage: {program} <file.cor>");
}

fn run(s: String) -> Result<String, MalError> {
    let ast = read_str(&s)?;
    let exp = eval_ast(ast)?;
    Ok(pr_str(&exp))
}

fn eval_ast(ast: MalType) -> MalResult {
    todo!()
}

// fn repl() {
//     let mut rl = Editor::<(), rustyline::history::DefaultHistory>::new().unwrap();
//     if rl.load_history(".mal-history").is_err() {
//         eprintln!("No previous history.");
//     }
//
//     loop {
//         let readline = rl.readline("user> ");
//         match readline {
//             Ok(line) => {
//                 let _ = rl.add_history_entry(&line);
//                 rl.save_history(".mal-history").unwrap();
//                 if !line.is_empty() {
//                     match rep(line) {
//                         Ok(out) => println!("{:?}", out),
//                         Err(e) => eprintln!("Error: {:?}", e),
//                     }
//                 }
//             }
//             Err(ReadlineError::Interrupted) => continue,
//             Err(ReadlineError::Eof) => break,
//             Err(err) => {
//                 println!("Error: {:?}", err);
//                 break;
//             }
//         }
//     }
// }
//
