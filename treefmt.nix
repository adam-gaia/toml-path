{
  projectRootFile = "flake.nix";
  programs = {
    alejandra.enable = true; # Nix formatter
    rustfmt.enable = true;
    just.enable = true;
    mdformat.enable = true;
    jsonfmt.enable = true;
    yamlfmt.enable = true;
    taplo.enable = true; # Toml formatter
    typos.enable = true;
  };
}
