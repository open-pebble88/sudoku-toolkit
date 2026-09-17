//! Parsing and structural checks for 9x9 sudoku boards.
//!
//! A board is plain text: nine lines of nine characters each, where
//! `1`-`9` are given digits and `.` (or `0`) marks an empty cell. Blank
//! lines are ignored so files can carry a trailing newline or spacing
//! without tripping the parser.

use std::fmt;

pub const SIZE: usize = 9;
const BOX_SIZE: usize = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    cells: [[u8; SIZE]; SIZE],
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    WrongRowCount(usize),
    WrongColumnCount { row: usize, found: usize },
    InvalidChar { row: usize, col: usize, ch: char },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::WrongRowCount(n) => {
                write!(f, "expected {} non-empty rows, found {}", SIZE, n)
            }
            ParseError::WrongColumnCount { row, found } => write!(
                f,
                "row {} has {} characters, expected {}",
                row + 1,
                found,
                SIZE
            ),
            ParseError::InvalidChar { row, col, ch } => write!(
                f,
                "invalid character '{}' at row {}, col {} (use 1-9 or '.')",
                ch,
                row + 1,
                col + 1
            ),
        }
    }
}

impl std::error::Error for ParseError {}

impl Board {
    /// Parses a board from text. Lines that are empty after trimming are
    /// skipped, so both `\n` and `\r\n` input, and files with a trailing
    /// blank line, parse the same way.
    pub fn parse(input: &str) -> Result<Board, ParseError> {
        let lines: Vec<&str> = input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();

        if lines.len() != SIZE {
            return Err(ParseError::WrongRowCount(lines.len()));
        }

        let mut cells = [[0u8; SIZE]; SIZE];
        for (row, line) in lines.iter().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            if chars.len() != SIZE {
                return Err(ParseError::WrongColumnCount {
                    row,
                    found: chars.len(),
                });
            }
            for (col, ch) in chars.iter().enumerate() {
                cells[row][col] = match ch {
                    '1'..='9' => *ch as u8 - b'0',
                    '.' | '0' => 0,
                    other => {
                        return Err(ParseError::InvalidChar {
                            row,
                            col,
                            ch: *other,
                        })
                    }
                };
            }
        }

        Ok(Board { cells })
    }

    /// Value at `(row, col)`, `0` meaning empty. Panics if out of range.
    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.cells[row][col]
    }

    /// True if no row, column, or 3x3 box has a repeated given digit.
    /// This does not require the board to be complete or solvable.
    pub fn is_valid(&self) -> bool {
        for row in 0..SIZE {
            if has_duplicate(self.cells[row]) {
                return false;
            }
        }
        for col in 0..SIZE {
            let column = std::array::from_fn(|row| self.cells[row][col]);
            if has_duplicate(column) {
                return false;
            }
        }
        for box_row in 0..BOX_SIZE {
            for box_col in 0..BOX_SIZE {
                let mut vals = [0u8; SIZE];
                let mut i = 0;
                for r in 0..BOX_SIZE {
                    for c in 0..BOX_SIZE {
                        vals[i] = self.cells[box_row * BOX_SIZE + r][box_col * BOX_SIZE + c];
                        i += 1;
                    }
                }
                if has_duplicate(vals) {
                    return false;
                }
            }
        }
        true
    }

    /// True if every cell holds a digit (no `0` entries left).
    pub fn is_complete(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|&v| v != 0))
    }
}

fn has_duplicate(vals: [u8; SIZE]) -> bool {
    let mut seen = [false; SIZE + 1];
    for v in vals {
        if v == 0 {
            continue;
        }
        if seen[v as usize] {
            return true;
        }
        seen[v as usize] = true;
    }
    false
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for &v in row {
                let ch = if v == 0 { '.' } else { (b'0' + v) as char };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_INCOMPLETE: &str = "\
53..7....
6..195...
.98....6.
8...6...3
4..8.3..1
7...2...6
.6....28.
...419..5
....8..79
";

    const ROW_CONFLICT: &str = "\
55.......
.........
.........
.........
.........
.........
.........
.........
.........
";

    #[test]
    fn parses_valid_incomplete_board() {
        let board = Board::parse(VALID_INCOMPLETE).unwrap();
        assert!(board.is_valid());
        assert!(!board.is_complete());
        assert_eq!(board.get(0, 0), 5);
        assert_eq!(board.get(0, 2), 0);
    }

    #[test]
    fn detects_row_conflict() {
        let board = Board::parse(ROW_CONFLICT).unwrap();
        assert!(!board.is_valid());
    }

    #[test]
    fn rejects_wrong_row_count() {
        let err = Board::parse("53..7....\n").unwrap_err();
        assert_eq!(err, ParseError::WrongRowCount(1));
    }

    #[test]
    fn rejects_bad_character() {
        let text = VALID_INCOMPLETE.replace('5', "x");
        let err = Board::parse(&text).unwrap_err();
        assert!(matches!(err, ParseError::InvalidChar { .. }));
    }

    #[test]
    fn round_trips_through_display() {
        let board = Board::parse(VALID_INCOMPLETE).unwrap();
        let printed = board.to_string();
        let reparsed = Board::parse(&printed).unwrap();
        assert_eq!(board, reparsed);
    }
}
