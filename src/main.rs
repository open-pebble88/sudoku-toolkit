use std::env;
use std::fs;
use std::io::{self, Read};
use std::process::ExitCode;

use sudoku_toolkit::Board;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let path = args.next();

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
