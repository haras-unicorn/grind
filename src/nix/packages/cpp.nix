{
  self.overlays.overlays.grind-cpp = {
    flake = true;
    value =
      final: _prev:
      let
        lib = final.lib;

        root = ../../..;
      in
      {
        grind-cpp = final.stdenv.mkDerivation {
          name = "grind-cpp";

          src = lib.fileset.toSource {
            inherit root;
            fileset = lib.fileset.unions [
              (root + "/CMakeLists.txt")
              ../../cpp
            ];
          };

          nativeBuildInputs = [
            final.cmake
            final.ninja
          ];

          # NOTE: the tests run in the dev shell checks, against the checkout,
          # so the package does not need doctest
          cmakeFlags = [ (lib.cmakeBool "GRIND_TESTS" false) ];

          # NOTE: with no solutions CMake has no install rules, and then ninja
          # has no install target, but cmake --install works regardless - and a
          # package has to have an output
          installPhase = ''
            runHook preInstall

            mkdir -p "$out"
            cmake --install .

            runHook postInstall
          '';

          meta = {
            description = "C++ solutions to small coding problems";
            license = lib.licenses.mit;
          };
        };
      };
  };
}
