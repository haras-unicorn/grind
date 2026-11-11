# Solving a problem

Start from the template of the language:

```console
dev new rust aoc-2025-day-11
dev new python leetcode-0001-two-sum
dev new cpp leetcode-0001-two-sum
```

The name is in kebab case. The same problem can be solved in more than one
language, because every language has its own directory.

Then write the solution in the copied file, and the problem statement or a link
to it in the `README.md` next to it.

## Checking one problem

`dev check` checks the whole repository. To check only the problem you are
working on, name it:

```console
dev check-problem rust aoc-2025-day-11
dev check-problem python leetcode-0001-two-sum
dev check-problem cpp leetcode-0001-two-sum
```

It runs the lint, type, test and spelling steps for that problem: clippy and
`cargo test` for Rust, ruff, basedpyright and pytest for Python, and a CMake
build with ctest and the sanitizers for C++. It does not check the formatting;
`dev format` does that for everything.

## Rust

The solution is a binary crate in the Cargo workspace. Add its dependencies to
`[workspace.dependencies]` in the root `Cargo.toml` first, and lock them.

```console
cargo run -p aoc-2025-day-11 < input.txt
cargo test -p aoc-2025-day-11
```

## Python

The solution is a script on the standard library. The tests are functions named
`test_*` that pytest finds in every file.

```console
python src/python/leetcode-0001-two-sum/main.py < input.txt
python -m pytest src/python/leetcode-0001-two-sum
```

## C++

The solution is a C++23 source file on the standard library. CMake builds two
binaries from it: the solution, and a test binary that defines `GRIND_TEST` and
compiles the [doctest](https://github.com/doctest/doctest) cases at the bottom
of the file with the address and undefined behavior sanitizers. The tests are
behind `#ifdef GRIND_TEST`, so the file can be submitted as it is.

The test binary also defines `DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN`, which gives
it a `main` from doctest, so the `main` of the solution is behind
`#ifndef DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN`. CMake also has a third target per
problem, `<problem>-editor`, that defines only `GRIND_TEST`. It is never built.
It exists so that clangd, which uses the first compile command of a file, sees
the tests and the `main` together.

```console
cmake -B build -G Ninja
cmake --build build
./build/leetcode-0001-two-sum < input.txt
ctest --test-dir build
```

CMake finds new problems by itself on the next build.

The dev shell runs the configure step when it starts, because the editor's
language server (clangd) reads `build/compile_commands.json` and cannot find the
standard library headers without it. After `dev new cpp`, build once or enter
the shell again so the new problem is in it. `dev check-problem` builds in
`build/check`, which clangd does not read.
