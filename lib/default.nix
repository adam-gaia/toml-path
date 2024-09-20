{
  flake,
  inputs,
  ...
}: let
  mkCrateBuilder = pkgs: let
    inherit (pkgs) lib;
    craneLib = inputs.crane.mkLib pkgs;
    src = craneLib.cleanCargoSource ../.;

    # Common arguments can be set here to avoid repeating them later
    commonArgs = {
      inherit src;
      strictDeps = true;

      buildInputs =
        [
          # Add additional build inputs here
        ]
        ++ lib.optionals pkgs.stdenv.isDarwin [
          # Additional darwin specific inputs can be set here
          pkgs.libiconv
        ];

      # Additional environment variables can be set directly
      # MY_CUSTOM_VAR = "some value";
    };
  in {
    inherit src lib craneLib commonArgs;
  };
in {
  inherit mkCrateBuilder;
}
