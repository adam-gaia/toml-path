{
  inputs,
  pkgs,
  system,
  ...
}: let
  crateBuilder = inputs.self.lib.mkCrateBuilder pkgs;
  commonArgs = crateBuilder.commonArgs;
  craneLib = crateBuilder.craneLib;

  craneLibLLvmTools =
    craneLib.overrideToolchain
    (inputs.fenix.packages.${system}.complete.withComponents [
      "cargo"
      "llvm-tools"
      "rustc"
    ]);

  # Build *just* the cargo dependencies, so we can reuse
  # all of that work (e.g. via cachix) when running in CI
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
  craneLibLLvmTools.cargoLlvmCov (commonArgs
    // {
      inherit cargoArtifacts;
    })
# TODO: how do we constrain this codecov package to linux only without constraining the main package?
#lib.optionalAttrs (!pkgs.stdenv.isDarwin) {
#my-crate-llvm-coverage =
#craneLibLLvmTools.cargoLlvmCov (commonArgs
#// {
#inherit cargoArtifacts;
#})
#};

