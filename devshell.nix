{
  inputs,
  pkgs,
  perSystem,
}: let
  crateBuilder = inputs.self.lib.mkCrateBuilder pkgs;
  craneLib = crateBuilder.craneLib;
  lib = crateBuilder.lib;
  commonArgs = crateBuilder.commonArgs;

  # Treefmt doesn't easily expose the programs with out its flake-parts module (as far as I can tell)
  # This snipit, modified from their default.nix, lets us grab the programs after building with our treefmt config
  treefmt-module-builder = nixpkgs: configuration: let
    mod = inputs.treefmt-nix.lib.evalModule nixpkgs configuration;
  in
    mod.config.build;
  treefmt-module = treefmt-module-builder pkgs (import ./treefmt.nix);
  treefmt-bin = treefmt-module.wrapper;
  treefmt-programs = lib.attrValues treefmt-module.programs;

  # Grab cargo, clippy, rustfmt, etc from crane's devShell to put in our own
  craneToolchain = (craneLib.devShell {}).nativeBuildInputs;
in
  inputs.devenv.lib.mkShell {
    inherit inputs pkgs;
    modules = [
      {
        packages = with pkgs;
          [
            rust-analyzer
            cargo-limit
            cargo-nextest
            cargo-dist
            just
            bacon
            oranda
            typos
            perSystem.cargo-nextest-xdg.default
          ]
          # Include the extra packages we use to build our crate
          ++ commonArgs.buildInputs
          # Include crane's toolchain (cargo, clippy, rustfmt, etc)
          ++ craneToolchain
          # Include treefmt and formatters
          ++ treefmt-programs
          ++ [treefmt-bin];

        enterShell = ''
          just --list
        '';

        pre-commit.hooks = {
          treefmt = {
            enable = true;
            package = treefmt-bin;
          };
        };
      }
    ];
  }
#craneLib.devShell {
# Inherit inputs from checks.
#checks = self.checks.${system}; TODO
# Additional dev-shell environment variables can be set directly
# MY_CUSTOM_DEVELOPMENT_VAR = "something else";
# Extra inputs can be added here; cargo and rustc are provided by default.
#}

