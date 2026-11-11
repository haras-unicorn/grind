{
  description = "Solutions to small coding problems from interviews and competitions";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";

    haraspkgs.url = "github:haras-unicorn/haraspkgs/v1.5.2";
    haraspkgs.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    { haraspkgs, ... }@inputs:
    haraspkgs.lib.makeFlake {
      inherit inputs;
      modules = [ ./src/nix ];
    };
}
