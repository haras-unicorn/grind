{ inputs, ... }:

{
  self.overlays.overlays.grind-rust = {
    flake = true;
    value =
      final: _prev:
      let
        lib = final.lib;

        root = ../../..;

        workspaceManifest = builtins.fromTOML (
          builtins.readFile (root + "/Cargo.toml")
        );

        # NOTE: only linux gets the musl target - there is no static darwin
        isLinux = final.stdenv.hostPlatform.isLinux;
        target = "${final.stdenv.hostPlatform.parsed.cpu.name}-unknown-linux-musl";
        targetUnderscored = builtins.replaceStrings [ "-" ] [ "_" ] target;

        musl = final.pkgsStatic.stdenv.cc;
        muslCc = "${musl}/bin/${musl.targetPrefix}cc";

        # NOTE: haraspkgs overlays are not available to downstream overlays
        # so this is what its rust helpers do, using its inputs
        rustBin = inputs.haraspkgs.inputs.rust-overlay.lib.mkRustBin { } final;

        toolchain = rustBin.stable.latest.default.override {
          extensions = [
            "rustfmt"
            "clippy"
            "rust-analyzer"
            # NOTE: rust-analyzer reads the standard library source for its types
            "rust-src"
          ];
          targets = lib.optional isLinux target;
        };

        craneLib = (inputs.haraspkgs.inputs.crane.mkLib final).overrideToolchain (
          _: toolchain
        );

        # NOTE: the advent of code solutions embed their input.txt with
        # include_str!, so it has to be in the source with the cargo files
        src = lib.cleanSourceWith {
          src = lib.fileset.toSource {
            inherit root;
            fileset = lib.fileset.unions [
              (root + "/Cargo.toml")
              (root + "/Cargo.lock")
              ../../crates
            ];
          };
          filter =
            path: type:
            (craneLib.filterCargoSources path type) || (lib.hasSuffix ".txt" path);
          name = "source";
        };

        # NOTE: used by the dev shell to write .cargo/config.toml so that
        # cargo works offline
        cargoVendorDir = craneLib.vendorCargoDeps { inherit src; };

        commonArgs = {
          inherit src cargoVendorDir;
          pname = "grind-rust";
          inherit (workspaceManifest.workspace.package) version;
          strictDeps = true;
          cargoExtraArgs = "--workspace";
          # NOTE: the tests run in the dev shell checks, against the checkout
          doCheck = false;
          env = lib.optionalAttrs isLinux {
            CARGO_BUILD_TARGET = target;
            CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
            "CARGO_TARGET_${lib.toUpper targetUnderscored}_LINKER" = muslCc;
          };
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in
      {
        grind-rust = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;

            passthru = {
              inherit toolchain craneLib cargoVendorDir;
            };

            meta = {
              description = workspaceManifest.workspace.package.description;
              license = lib.licenses.mit;
            };
          }
        );
      };
  };
}
