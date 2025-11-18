{
  self,
  root,
  naersk,
  rust-overlay,
  ...
}:

let
  mkNaerskLib =
    pkgs:
    pkgs.callPackage naersk {
      cargo = pkgs.rust-naersk;
      rustc = pkgs.rust-naersk;
    };
in
{
  flake.lib.rust.overlays = [
    (import rust-overlay)
    (final: prev: {
      rust-naersk = prev.rust-bin.stable.latest.default.override {
        extensions = [
          "clippy"
          "rustfmt"
          "rust-analyzer"
          "rust-src"
        ];
      };
    })
  ];

  flake.lib.rust.mkPackage =
    {
      pkgs,
      crate,
      version ? "1.0.0",
      isExe ? false,
    }:
    let
      naerskLib = mkNaerskLib pkgs;
    in
    naerskLib.buildPackage (
      {
        inherit root;
        name = "grind-${crate}";
        pname = "grind-${crate}";
        version = version;
        src = "${root}/src/${crate}";
      }
      // (
        if isExe then
          {
            meta.mainProgram = crate;
          }
        else
          { }
      )
    );

  flake.lib.rust.mkDevShell =
    pkgs:
    pkgs.mkShell {
      shellHook = ''
        export RUST_BACKTRACE="full";
      '';

      packages = with pkgs; [
        llvmPackages.clangNoLibcxx
        lldb
        rust-naersk
        cargo-edit
        evcxr
      ];
    };
}
