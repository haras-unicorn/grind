{
  self.overlays.overlays.grind-docs = {
    flake = true;
    value = final: _prev: {
      grind-docs =
        final.runCommand "grind-docs"
          {
            src = final.lib.fileset.toSource {
              root = ../../..;
              fileset = ../../../docs;
            };
            nativeBuildInputs = [ final.mdbook ];
          }
          ''
            mdbook build -d "$out" "$src/docs"
          '';
    };
  };
}
