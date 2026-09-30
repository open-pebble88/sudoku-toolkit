use std::env;
use std::fs;
use std::io::{self, Read};
use std::process::ExitCode;

use sudoku_toolkit::Board;

const USAGE: &str = "usage: sudoku [--solve] [FILE | -]";

fn main() -> ExitCode {
    let mut solve = false;
    let mut path: Option<String> = None;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--solve" => solve = true,
            // a lone "-" means stdin, so it is a path, not a flag
            flag if flag.starts_with('-') && flag != "-" => {
                eprintln!("unknown option: {}", flag);
                eprintln!("{}", USAGE);
                return ExitCode::FAILURE;
            }
            _ => {
                if path.is_some() {
                    eprintln!("only one input file may be given");
                    eprintln!("{}", USAGE);
                    return ExitCode::FAILURE;
                }
                path = Some(arg);
            }
        }
    }

    let input = match path.as_deref() {
        None | Some("-") => match read_stdin() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error reading stdin: {}", e);
                return ExitCode::FAILURE;
            }
        },
        Some(path) => match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error reading {}: {}", path, e);
                return ExitCode::FAILURE;
            }
        },
    };

    let board = match Board::parse(&input) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("could not parse board: {}", e);
            return ExitCode::FAILURE;
        }
    };

    if solve {
        return match board.solve() {
            Some(solution) => {
                print!("{}", solution);
                ExitCode::SUCCESS
            }
            None => {
                eprintln!("no solution");
                ExitCode::FAILURE
            }
        };
    }

    print!("{}", board);
    println!("valid:    {}", board.is_valid());
    println!("complete: {}", board.is_complete());

    if board.is_valid() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn read_stdin() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}
