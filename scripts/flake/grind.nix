{
  self,
  root,
  lib,
  nixpkgs,
  perch,
  ...
}:

{
  flake.packages =
    let
      crates = builtins.map (item: item.name) (
        builtins.filter (
          item: item.value == "directory" && builtins.pathExists "${root}/src/${item.name}/Cargo.toml"
        ) (lib.attrsToList (builtins.readDir "${root}/src"))
      );
    in
    builtins.listToAttrs (
      builtins.map (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = self.lib.rust.overlays;
          };
        in
        {
          name = system;
          value = builtins.listToAttrs (
            builtins.map (crate: {
              name = crate;
              value = self.lib.rust.mkPackage {
                inherit pkgs crate;
                isExe = true;
              };
            }) crates
          );
        }
      ) perch.lib.defaults.systems
    );
}
