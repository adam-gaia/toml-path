{
  inputs,
  pkgs,
  perSystem,
}: let
  crateBuilder = inputs.self.lib.mkCrateBuilder pkgs;
  craneLib = crateBuilder.craneLib;
  lib = crateBuilder.lib;

  # Treefmt doesn't easily expose the programs with out its flake-parts module (as far as I can tell)
  # This snipit, modified from their default.nix, lets us grab the programs after buiding with our treefmt config
  treefmt-module-builder = nixpkgs: configuration: let
    mod = inputs.treefmt-nix.lib.evalModule nixpkgs configuration;
  in
    mod.config.build;
  treefmt-module = treefmt-module-builder pkgs (import ./treefmt.nix);
  treefmt-bin = treefmt-module.wrapper;
  treefmt-programs = lib.attrValues treefmt-module.programs;
in
  craneLib.devShell {
    # Inherit inputs from checks.
    #checks = self.checks.${system}; TODO

    # Additional dev-shell environment variables can be set directly
    # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

    # Extra inputs can be added here; cargo and rustc are provided by default.
    packages = with pkgs;
      [
        cargo-limit
        cargo-nextest
        just
        bacon
        cargo-dist
        oranda
        treefmt-bin
        perSystem.cargo-nextest-xdg.default
      ]
      ++ treefmt-programs;
  }
