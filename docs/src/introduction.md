# Introduction

My solutions to small coding problems: interview questions, competitions and
puzzles like [Advent of Code](https://adventofcode.com) and
[LeetCode](https://leetcode.com).

Every problem is a directory with a single source file that reads the input from
stdin and prints the answer to stdout, which is what judges expect. Its unit
tests sit at the bottom of that same file.

## Languages

| Language | Problems            | Solution      | Built by |
| -------- | ------------------- | ------------- | -------- |
| Rust     | `src/crates/<name>` | `src/main.rs` | Cargo    |
| Python   | `src/python/<name>` | `main.py`     | none     |
| C++      | `src/cpp/<name>`    | `main.cpp`    | CMake    |

Each language has a template in `assets/templates/<language>`, which `dev new`
copies to start a new problem. The templates are not part of the code, so the
linters, formatters and checks leave them alone. See
[solving a problem](solving.md).
