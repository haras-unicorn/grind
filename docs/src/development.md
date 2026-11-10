# Development

The repository is a Nix flake built on
[haraspkgs](https://github.com/haras-unicorn/haraspkgs). Its dev shell has the
toolchain of every language, the language servers and the `dev` command.

```console
nix develop
```

With direnv, the shell loads when you enter the directory.

## Commands

| Command             | What it does                                     |
| ------------------- | ------------------------------------------------ |
| `dev new`           | Copies the template of a language.               |
| `dev check`         | Runs every check: lints, types, tests, spelling. |
| `dev check-problem` | Runs the same checks for one problem only.       |
| `dev format`        | Formats the whole repository.                    |
| `dev docs`          | Builds this book.                                |
| `nix flake check`   | Runs the checks in the Nix sandbox, like CI.     |

## Packages

Each language builds into one package with a binary per problem: `grind-rust`,
`grind-python` and `grind-cpp`. The Rust binaries are static and linked with
musl on Linux.

```console
nix build .#grind-rust
./result/bin/aoc-2024-day-01
```

## Documentation

This book is built by Nix and published to GitHub Pages by the docs workflow on
every push to `main`. The repository's Pages source has to be set to GitHub
Actions for that to work.

## Generated files

The dev shell writes `.gitignore`, `.gitattributes`, `.envrc`, `LICENSE.md`,
`CODE_OF_CONDUCT.md` and the GitHub workflows when it starts. They are committed
so tools that run before the shell can read them, but the source of truth is
`src/nix/dev/shell`. Edit that and not the files.
