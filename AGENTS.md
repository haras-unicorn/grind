# grind

Solutions to small coding problems from interviews and competitions, like Advent
of Code and LeetCode, in Rust, Python and C++. Every problem is a directory with
one source file that reads stdin and prints the answer to stdout.

## Structure

- `src/crates/<problem>`: Rust solutions, members of the Cargo workspace in the
  root `Cargo.toml`.
- `src/python/<problem>`: Python solutions, a `main.py` each. Tool settings are
  in the root `pyproject.toml`.
- `src/cpp/<problem>`: C++ solutions, a `main.cpp` each, built by the root
  `CMakeLists.txt`, which finds every directory by itself.
- `assets/templates/<language>`: the problem `dev new` copies into the directory
  of that language. Nothing lints, formats, checks or builds them, and the Cargo
  workspace excludes the Rust one.
- `src/nix`: flake modules for haraspkgs. `packages/` has one package per
  language and the docs, and `dev/shell/` has the dev shell with its assets.
- `docs`: the mdbook.

## Development

The dev shell (`nix develop`, or direnv) has every toolchain and the `dev`
command: `dev check`, `dev format`, `dev new <language> <problem>`,
`dev check-problem <language> <problem>` and `dev docs`. `nix flake check` runs
the same checks in the Nix sandbox, which is what the check workflow does, along
with building the packages.

Cargo works offline: the shell links `.cargo/config.toml` to the crates vendored
by Nix from `Cargo.lock`, so a new Rust dependency only works after the human
locks it.

## Generated files

`.gitignore`, `.gitattributes`, `.envrc`, `LICENSE.md`, `CODE_OF_CONDUCT.md` and
`.github/workflows/` are written by the dev shell from
`src/nix/dev/shell/default.nix`. Edit that and not the files. The EditorConfig,
cspell and Helix configs are not committed at all; the wrapped tools get them.

## Deviations for the whole repository

- A solution is one file for stdin and stdout, because that is what judges take.
  So there is no app skeleton (arguments, config, logs, async runtime) and no
  library crate, and printing the answer is allowed where it happens, with a
  reason.
- The unit tests sit at the bottom of the solution in every language. In C++
  they are behind `#ifdef GRIND_TEST`, so the file stays submittable.
- Python and C++ use only the standard library, plus pytest and doctest for the
  tests, because that is what judges allow. So there is no uv lock and no C++
  package manager. If a solution ever needs more, the haraspkgs uv helpers are
  the way for Python.
- The `aoc-*` crates predate this setup. They keep their own package metadata
  (edition 2021) and crate-level lint attributes, and do not take the workspace
  lints. New crates start from the template and follow the workspace.
