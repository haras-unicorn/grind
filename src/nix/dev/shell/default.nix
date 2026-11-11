{ inputs, selfLib, ... }:

{
  self.devShells.default = {
    flake = true;
    value =
      {
        config,
        lib,
        pkgs,
        ...
      }:
      let
        # NOTE: the package set of a dev shell only has some of the haraspkgs
        # overlays, and the functions among them are not flake packages, so
        # they are taken from the composed result of haraspkgs
        inherit (inputs.haraspkgs.lib.${pkgs.stdenv.hostPlatform.system}.overlays.other)
          writeNushellApplication
          ;

        yaml = pkgs.formats.yaml { };

        toml = pkgs.formats.toml { };

        json = pkgs.formats.json { };

        harasPackages = pkgs.harasPackages;

        selfPackages = pkgs.${selfLib.overlays.namespace};

        # NOTE: the toolchain and vendored crates come from the package so the
        # dev shell and the nix build always agree
        inherit (selfPackages.grind-rust.passthru) toolchain cargoVendorDir;

        # NOTE: the python of the package, so the dev shell and the nix build
        # always agree - pytest is only needed by the tests
        python =
          selfPackages.grind-python.passthru.python.withPackages
            (pythonPackages: [ pythonPackages.pytest ]);

        # NOTE: the width and indent of the editorconfig, for the tools that do
        # not read it
        lineWidth = 80;
        indentWidth = 2;

        # NOTE: passed on the command line, so neither the formatter nor the
        # check needs a rustfmt.toml
        rustfmtConfig = "max_width=${builtins.toString lineWidth},tab_spaces=${builtins.toString indentWidth}";

        rustfmtOptions = [
          "--edition"
          "2024"
          "--config"
          rustfmtConfig
        ];

        # NOTE: a check runs on a copy of the repository without the generated
        # .cargo/config.toml, so this points cargo at the crates vendored by nix
        # the same way, and it needs no network access
        cargoOffline = ''
          CARGO_HOME="$(mktemp -d)"
          export CARGO_HOME
          trap 'rm -rf "$CARGO_HOME"' EXIT
          cp ${cargoVendorDir}/config.toml "$CARGO_HOME/config.toml"
        '';

        templates = {
          rust = {
            source = "assets/templates/rust";
            target = "src/crates";
          };
          python = {
            source = "assets/templates/python";
            target = "src/python";
          };
          cpp = {
            source = "assets/templates/cpp";
            target = "src/cpp";
          };
        };

        newProblem = writeNushellApplication {
          name = "new-problem";
          environment.GRIND_TEMPLATES = builtins.toJSON templates;
          text = builtins.readFile ./new.nu;
        };

        prettierConfig = yaml.generate "prettier.yaml" {
          printWidth = lineWidth;
          tabWidth = indentWidth;
          useTabs = false;
          trailingComma = "all";
          endOfLine = "lf";
          proseWrap = "always";
        };

        cspellConfig = yaml.generate "cspell.yaml" {
          useGitignore = true;
          version = "0.2";
          language = "en";
          ignorePaths = [
            "CODE_OF_CONDUCT.md"
            "Cargo.lock"
            "flake.lock"
            "**/input.txt"
            "assets/**"
          ];
          words = builtins.fromJSON (builtins.readFile ./words.json);
        };

        markdownlintConfig = json.generate "markdownlint.json" {
          "$schema" =
            "https://raw.githubusercontent.com/DavidAnson/markdownlint/main/schema/markdownlint-config-schema.json";
          # NOTE: prettier pads tables and keeps long code lines and links
          MD013 = {
            tables = false;
            code_blocks = false;
          };
        };

        helixServers = lib.mapAttrs (
          _: server:
          {
            inherit (server) command;
          }
          // lib.optionalAttrs (server.arguments != [ ]) {
            args = server.arguments;
          }
          // lib.optionalAttrs (server.config != { }) {
            inherit (server) config;
          }
        ) config.languages.servers;

        helixLanguages = lib.mapAttrsToList (
          name: language:
          {
            inherit name;
            auto-format = language.auto-format;
          }
          // lib.optionalAttrs (language.servers != [ ]) {
            language-servers = language.servers;
          }
          // lib.optionalAttrs (language.formatter != null) {
            formatter = {
              inherit (language.formatter) command;
            }
            // lib.optionalAttrs (language.formatter.arguments != [ ]) {
              args = language.formatter.arguments;
            };
          }
        ) config.languages.languages;

        helixLanguagesToml = toml.generate "languages.toml" (
          lib.optionalAttrs (helixServers != { }) {
            language-server = helixServers;
          }
          // lib.optionalAttrs (helixLanguages != [ ]) {
            language = helixLanguages;
          }
        );

        helixConfigToml = toml.generate "config.toml" {
          editor = {
            text-width = lineWidth;
            default-line-ending = "lf";
            insert-final-newline = true;
          };
        };

        nixfmtWrapped = harasPackages.makeBubblewrapWrapper {
          name = "nixfmt";
          package = pkgs.nixfmt;
        };

        prettierWrapped = harasPackages.makeBubblewrapWrapper {
          name = "prettier";
          package = pkgs.prettier;
        };

        taploWrapped = harasPackages.makeBubblewrapWrapper {
          name = "taplo";
          package = pkgs.taplo;
        };

        cspellWrapped = harasPackages.makeBubblewrapWrapper {
          name = "cspell";
          package = pkgs.cspell;
        };

        hxWrapped = harasPackages.makeBubblewrapWrapper {
          name = "hx";
          package = pkgs.helix;
          files = {
            "~/.cache/helix" = {
              mode = "rw";
            };
            ".helix/languages.toml" = {
              mode = "ro";
              source = helixLanguagesToml;
            };
            ".helix/config.toml" = {
              mode = "ro";
              source = helixConfigToml;
            };
          };
        };

        cppPackages = [
          pkgs.cmake
          pkgs.ninja
          pkgs.doctest
        ];

        checkProblem = writeNushellApplication {
          name = "check-problem";
          runtimeInputs = [
            toolchain
            pkgs.stdenv.cc
            python
            pkgs.ruff
            pkgs.basedpyright
            cspellWrapped
          ]
          ++ cppPackages;
          environment = {
            GRIND_TEMPLATES = builtins.toJSON templates;
            GRIND_CHECK_BUILD = "build/check";
            # NOTE: cmake finds doctest through this, which the setup hook of
            # the dev shell would otherwise do
            CMAKE_PREFIX_PATH = "${pkgs.doctest}";
          };
          text = builtins.readFile ./check.nu;
        };

        external = [
          pkgs.nil
          pkgs.nix
          pkgs.fd
          pkgs.nushell
          pkgs.yaml-language-server
          pkgs.vscode-langservers-extracted
          pkgs.markdownlint-cli
          pkgs.marksman
          pkgs.mdbook
          pkgs.lldb
          pkgs.cargo-edit
          pkgs.evcxr
          pkgs.ruff
          pkgs.basedpyright
          pkgs.clang-tools
        ]
        ++ cppPackages;
      in
      {
        packages = [
          harasPackages.flake-root
          toolchain
          python
          nixfmtWrapped
          prettierWrapped
          taploWrapped
          cspellWrapped
          hxWrapped
        ]
        ++ external;

        env.RUST_BACKTRACE = "full";

        # NOTE: clangd reads the compile commands that cmake writes into build,
        # so this is what makes the editor work on a fresh machine. It only
        # warns on failure, because it must not keep the shell from starting
        shellHook = ''
          (
            cd "$(flake-root)"
            cmake -S . -B build -G Ninja \
              -DCMAKE_PREFIX_PATH=${pkgs.doctest} > /dev/null
          ) || echo "warning: could not configure the cmake build" >&2
        '';

        bwrap.files."cspell.yaml".source = cspellConfig;

        # NOTE: points cargo at the crates vendored by nix so it works without
        # network access - ignored by git, and not part of the nix build source
        file.".cargo/config.toml".source = "${cargoVendorDir}/config.toml";

        git = {
          enable = true;
          ignore = {
            templates = [
              "Nix"
              "Rust"
              "Python"
              "C++"
              "CMake"
            ];
            patterns = ''
              # Repository

              .env
              docs/book
              /build
            '';
          };
          attributes = ''
            * text=auto eol=lf
          '';
        };

        gitleaks.enable = true;

        direnv = {
          enable = true;
          extra = "dotenv_if_exists .env";
        };

        editorconfig = {
          enable = true;
          settings."*" = {
            end_of_line = "lf";
            charset = "utf-8";
            max_line_length = lineWidth;
            indent_style = "space";
            indent_size = indentWidth;
            tab_width = indentWidth;
            insert_final_newline = true;
            trim_trailing_whitespace = true;
          };
        };

        languages = {
          servers = {
            nil.command = "nil";

            # NOTE: rust-analyzer formats with rustfmt, so this is how the
            # editor gets the width and indent - it passes the edition itself
            rust-analyzer = {
              command = "rust-analyzer";
              config.rustfmt.extraArgs = [
                "--config"
                rustfmtConfig
              ];
            };

            basedpyright = {
              command = "basedpyright-langserver";
              arguments = [ "--stdio" ];
            };

            ruff = {
              command = "ruff";
              arguments = [ "server" ];
            };

            # NOTE: clangd asks the compiler in compile_commands.json for its
            # include paths, and in nix it has to be told which one it can ask
            clangd = {
              command = "clangd";
              arguments = [
                "--query-driver=${pkgs.stdenv.cc}/bin/*"
                "--fallback-style=LLVM"
              ];
            };

            taplo = {
              command = "taplo";
              arguments = [
                "lsp"
                "stdio"
              ];
            };

            yaml-language-server = {
              command = "yaml-language-server";
              arguments = [ "--stdio" ];
            };

            vscode-json-language-server = {
              command = "vscode-json-language-server";
              arguments = [ "--stdio" ];
              config = {
                provideFormatter = true;
                format.enable = true;
                json.validate.enable = true;
              };
            };

            marksman = {
              command = "marksman";
              arguments = [ "server" ];
            };

            nu-lsp = {
              command = "nu";
              arguments = [ "--lsp" ];
            };
          };

          languages = {
            nix = {
              servers = [ "nil" ];
              formatter = {
                command = lib.getExe pkgs.nixfmt;
                arguments = [
                  "--width"
                  (builtins.toString lineWidth)
                  "--filename"
                  "%{buffer_name}"
                ];
              };
            };

            rust.servers = [ "rust-analyzer" ];

            # NOTE: ruff is the one that formats, basedpyright only checks types
            python.servers = [
              "basedpyright"
              "ruff"
            ];

            cpp.servers = [ "clangd" ];

            toml.servers = [ "taplo" ];

            yaml = {
              servers = [ "yaml-language-server" ];
              formatter = {
                command = lib.getExe pkgs.prettier;
                arguments = [
                  "--config"
                  (builtins.toString prettierConfig)
                  "--parser"
                  "yaml"
                ];
              };
            };

            json = {
              servers = [ "vscode-json-language-server" ];
              formatter = {
                command = lib.getExe pkgs.prettier;
                arguments = [
                  "--config"
                  (builtins.toString prettierConfig)
                  "--parser"
                  "json"
                ];
              };
            };

            markdown = {
              servers = [ "marksman" ];
              formatter = {
                command = lib.getExe pkgs.prettier;
                arguments = [
                  "--config"
                  (builtins.toString prettierConfig)
                  "--parser"
                  "markdown"
                ];
              };
            };

            nu.servers = [ "nu-lsp" ];
          };
        };

        license = {
          enable = true;
          spdx = "MIT";
          path = "LICENSE.md";
          attrs = {
            "<year>" = "2025";
            "<copyright holders>" = "haras-unicorn <licensing@haras-unicorn.net>";
          };
        };

        codeOfConduct.contributorCovenant = {
          enable = true;
          path = "CODE_OF_CONDUCT.md";
        };

        cli.commands = {
          docs = {
            description = "Build the documentation.";
            packages = [ pkgs.nix ];
            commands = ''nix build ".#grind-docs" --no-link --print-out-paths'';
          };

          new = {
            description = "Copy the template of a language into a new problem, like `dev new rust aoc-2025-day-11`.";
            commands = ''${lib.getExe newProblem} "$@"'';
          };

          check-problem = {
            description = "Check only one problem, like `dev check-problem rust aoc-2025-day-11`.";
            commands = ''${lib.getExe checkProblem} "$@"'';
          };
        };

        workflows = {
          check = {
            on = {
              pull_request = { };
              push.branches = [ "main" ];
            };

            matrix.os = [ "ubuntu-latest" ];

            installable = null;

            steps = [
              {
                run = "nix flake check --show-trace --print-build-logs --max-jobs 1 --keep-going";
              }
              {
                run = "nix build --show-trace --print-build-logs .#grind-rust .#grind-python .#grind-cpp .#grind-docs";
              }
            ];
          };

          # NOTE: the book is built by nix and deployed to GitHub Pages. The
          # job needs the github-pages environment for the deploy action, and
          # the repository's Pages source set to GitHub Actions. The artifact
          # is a copy because the action does not follow the result link.
          docs = {
            on = {
              push.branches = [ "main" ];
              workflow_dispatch = { };
            };

            permissions = {
              pages = "write";
              "id-token" = "write";
            };

            installable = ".#grind-docs";

            steps = [
              { run = "cp -rL result site"; }
              { uses = "actions/configure-pages@v5"; }
              {
                uses = "actions/upload-pages-artifact@v3";
                "with".path = "site";
              }
              {
                id = "deployment";
                uses = "actions/deploy-pages@v4";
              }
            ];

            jobs.build = {
              environment = {
                name = "github-pages";
                url = "\${{ steps.deployment.outputs.page_url }}";
              };
              concurrency = {
                group = "pages";
                cancel-in-progress = false;
              };
            };
          };
        };

        check.checks = {
          rust-clippy = {
            packages = [
              toolchain
              pkgs.stdenv.cc
            ];
            commands = ''
              ${cargoOffline}
              cargo clippy --workspace --all-targets -- -D warnings
            '';
          };

          rust-test = {
            packages = [
              toolchain
              pkgs.stdenv.cc
            ];
            commands = ''
              ${cargoOffline}
              cargo test --workspace
            '';
          };

          python-lint = {
            packages = [ pkgs.ruff ];
            commands = "ruff check";
          };

          python-types = {
            packages = [
              pkgs.basedpyright
              python
            ];
            commands = "basedpyright";
          };

          # NOTE: pytest exits with 5 when it collects no tests, which is how it
          # is until the first python solution
          python-test = {
            packages = [ python ];
            commands = ''
              status=0
              python -m pytest || status=$?
              test "$status" -eq 0 -o "$status" -eq 5
            '';
          };

          # NOTE: builds in a temporary directory, so warnings as errors does
          # not stick to the build directory used in the dev shell
          cpp-test = {
            packages = cppPackages ++ [ pkgs.stdenv.cc ];
            commands = ''
              build="$(mktemp -d)"
              trap 'rm -rf "$build"' EXIT
              cmake -S . -B "$build" -G Ninja -DCMAKE_COMPILE_WARNING_AS_ERROR=ON
              cmake --build "$build"
              ctest --test-dir "$build" --output-on-failure
            '';
          };

          cspell = {
            packages = [ cspellWrapped ];
            commands = "cspell lint . --no-progress";
          };

          markdownlint = {
            packages = [ pkgs.markdownlint-cli ];
            commands = ''
              markdownlint --config ${markdownlintConfig} --ignore-path .gitignore --ignore CODE_OF_CONDUCT.md --ignore LICENSE.md --ignore docs/src/SUMMARY.md --ignore 'assets/**' .
            '';
          };

          nil = {
            packages = [
              pkgs.fd
              pkgs.nil
            ];
            commands = "fd --glob '*.nix' . -0 | xargs -0 -r nil diagnostics";
          };
        };

        format = {
          excludes = [
            "target/**"
            "build/**"
            "assets/**"
          ];

          formatters = {
            nixfmt = {
              packages = [ nixfmtWrapped ];
              command = "nixfmt";
              options = [
                "--width"
                (builtins.toString lineWidth)
              ];
              includes = [ "*.nix" ];
            };

            taplo = {
              packages = [ taploWrapped ];
              command = "taplo";
              options = [ "format" ];
              includes = [ "*.toml" ];
            };

            prettier = {
              packages = [ prettierWrapped ];
              command = "prettier";
              options = [
                "--write"
                "--config"
                (builtins.toString prettierConfig)
              ];
              includes = [
                "*.json"
                "*.md"
                "*.yaml"
                "*.yml"
              ];
              excludes = [
                "flake.lock"
                "CODE_OF_CONDUCT.md"
                "LICENSE.md"
              ];
            };

            rustfmt = {
              packages = [ toolchain ];
              command = "rustfmt";
              options = rustfmtOptions;
              includes = [ "*.rs" ];
            };

            ruff = {
              packages = [ pkgs.ruff ];
              command = "ruff";
              options = [ "format" ];
              includes = [ "*.py" ];
            };

            # NOTE: the llvm style already has the width and indent of the
            # editorconfig, and naming it keeps a .clang-format from a parent
            # directory out
            clang-format = {
              packages = [ pkgs.clang-tools ];
              command = "clang-format";
              options = [
                "-i"
                "--style=LLVM"
              ];
              includes = [
                "*.cpp"
                "*.hpp"
              ];
            };
          };
        };
      };
  };
}
