set windows-shell := ["nu.exe", "-c"]
set shell := ["nu", "-c"]

root := absolute_path('')

default:
    @just --choose

format:
    cd '{{ root }}'; just --unstable --fmt
    prettier --write '{{ root }}'
    nixfmt ...(fd '.*.nix$' '{{ root }}' | lines)
    cd '{{ root }}'; cargo fmt --all

lint:
    cd '{{ root }}'; just --unstable --fmt --check
    prettier --check '{{ root }}'
    cspell lint '{{ root }}' --no-progress
    nixfmt --check ...(fd '.*.nix$' '{{ root }}' | lines)
    markdownlint --ignore-path .gitignore '{{ root }}'
    if (markdown-link-check \
      --config '{{ root }}/.markdown-link-check.json' \
      ...(fd '^.*.md$' '{{ root }}' | lines) \
      | rg -q error \
      | complete \
      | get exit_code) == 0 { exit 1 }
    cd '{{ root }}'; cargo clippy -- -D warnings
