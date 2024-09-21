{
  flake,
  inputs,
  ...
}: let
  mkCrateBuilder = pkgs: let
    inherit (pkgs) lib;
    craneLib = inputs.crane.mkLib pkgs;

    src = let
        # This crate includes the README.md in the api docs. We need to include the readme.md
        # See https://crane.dev/source-filtering.html
        markdownFilter = path: _type: builtins.match ".*md$" path != null;
        markdownOrCargo = path: type: (markdownFilter path type) || (craneLib.filterCargoSources path type);
    in lib.cleanSourceWith {
      src = ../.; # The original, unfiltered source
      filter = markdownOrCargo;
      name = "source"; # Be reproducible, regardless of the directory name
    };

    #src = craneLib.cleanCargoSource ../.;

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
