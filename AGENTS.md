# grind

Solutions to small coding problems from interviews and competitions, like Advent
of Code and LeetCode, in Rust, Python and C++. Every problem is a directory with
one source file that reads stdin and prints the answer to stdout.

learning: true

The human writes the solutions, to learn. The `learning-project` skill applies
with this scope:

- The solutions are everything in `src/crates`, `src/python` and `src/cpp`, the
  old `aoc-*` crates too. The skill's hard limits apply there in full: no code,
  no fixes, no refactors.
- The one loosening is the unit tests at the bottom of a solution, which the
  assistant may write. The problems are well known, so the learning is in the
  solution and not in designing its tests. The tests are written to break the
  solution, and the code above them stays the human's.
- Questions about syntax and the standard library get a direct answer: the
  spelling, the header and a small example on other data. They are hard to
  search for and cost time. Nudge early towards the right API or idea and do not
  wait through a long chain of questions. The logic and the shape of the
  solution stay the human's.
- The tooling is allowed without asking, even where it holds code: `src/nix`,
  the templates in `assets/templates`, the root Cargo, CMake and Python
  configuration, the docs and the CI workflows.
- Practice problems come as a statement, an input format and examples. No
  approach, data structure names or traps, unless the human asks for a hint.

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

On entry the shell also configures CMake in `build/`, so clangd has its compile
commands without any manual step.

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
