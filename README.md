# sudoku-toolkit

A small Rust library for parsing and checking sudoku boards, plus a CLI
built on top of it.

## Board format

Nine lines, nine characters each. `1`-`9` are given digits, `.` (or `0`)
marks an empty cell. Blank lines around the grid are ignored, so a
trailing newline in a file won't break parsing.

```
53..7....
6..195...
.98....6.
8...6...3
4..8.3..1
7...2...6
.6....28.
...419..5
....8..79
```

## CLI usage

The CLI reads a board from a file path, or from stdin if no path is
given (or the path is `-`). It prints the board back, whether it's
structurally valid (no repeated digit in any row, column, or 3x3 box),
and whether every cell is filled in.

From a file:

```
$ sudoku board.txt
53..7....
6..195...
.98....6.
8...6...3
4..8.3..1
7...2...6
.6....28.
...419..5
....8..79
valid:    true
complete: false
```

From stdin, e.g. piped from another program or typed via a heredoc:

```
$ cat board.txt | sudoku
$ sudoku - < board.txt
$ sudoku <<'EOF'
53..7....
6..195...
.98....6.
8...6...3
4..8.3..1
7...2...6
.6....28.
...419..5
....8..79
EOF
```

Exit status is 0 when the board is structurally valid, 1 when it's
either malformed (parse error) or has a duplicate.

## Library usage

```rust
use sudoku_toolkit::Board;

let text = std::fs::read_to_string("board.txt")?;
let board = Board::parse(&text)?;

assert!(board.is_valid());
println!("{}", board.get(0, 0)); // 5

let solution = board.solve().expect("has a solution");
assert!(solution.is_complete());
```

## What's here

- `src/lib.rs`: `Board`, parsing, the row/column/box validity check, and
  a backtracking `solve()`.
- `src/main.rs`: the `sudoku` binary — file-or-stdin input, prints a
  short report.

## Not here yet

`solve()` isn't wired up to the CLI yet, so there's no `--solve` flag.
There's also no puzzle generator, and no check for a *unique* solution —
`solve()` just returns the first one it finds.

## Building

Standard library only, no external crates.

```
cargo build
cargo test
cargo run -- board.txt
```

## License

MIT, see `LICENSE`.
