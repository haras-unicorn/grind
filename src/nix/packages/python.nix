{
  self.overlays.overlays.grind-python = {
    flake = true;
    value =
      final: _prev:
      let
        lib = final.lib;

        root = ../../..;

        pyproject = builtins.fromTOML (builtins.readFile (root + "/pyproject.toml"));

        python = final.python3;
      in
      {
        # NOTE: every solution is one script on the standard library, so there
        # is nothing to build - each gets a wrapper that runs it in isolated
        # mode, so the environment cannot change what it imports
        grind-python = final.stdenvNoCC.mkDerivation {
          pname = "grind-python";
          inherit (pyproject.project) version;

          src = lib.fileset.toSource {
            inherit root;
            fileset = ../../python;
          };

          nativeBuildInputs = [ final.makeWrapper ];

          installPhase = ''
            runHook preInstall

            shopt -s nullglob
            mkdir -p "$out/bin" "$out/share"
            cp -r src/python "$out/share/grind-python"

            for problem in "$out/share/grind-python"/*/; do
              problem="''${problem%/}"
              makeWrapper ${lib.getExe python} "$out/bin/$(basename "$problem")" \
                --add-flags "-I $problem/main.py"
            done

            runHook postInstall
          '';

          passthru = {
            inherit python;
          };

          meta = {
            inherit (pyproject.project) description;
            license = lib.licenses.mit;
          };
        };
      };
  };
}
