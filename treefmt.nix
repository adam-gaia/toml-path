{
  projectRootFile = "flake.nix";
  programs = {
    alejandra.enable = true; # Nix formatter
    rustfmt.enable = true;
    just.enable = true;
  };
}
