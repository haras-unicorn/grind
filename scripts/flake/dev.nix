{ self, pkgs, ... }:

{
  seal.defaults.devShell = "dev";
  integrate.devShell = {
    nixpkgs.overlays = self.lib.rust.overlays;
    devShell = pkgs.mkShell {
      inputsFrom = [
        (self.lib.rust.mkDevShell pkgs)
      ];

      packages = with pkgs; [
        # Nix
        nil
        nixfmt-rfc-style

        # Scripts
        just
        nushell

        # Misc
        nodePackages.prettier
        nodePackages.yaml-language-server
        nodePackages.vscode-langservers-extracted
        markdownlint-cli
        nodePackages.markdown-link-check
        marksman
        taplo

        # Tools
        nodePackages.cspell
        trufflehog
      ];
    };
  };
}
