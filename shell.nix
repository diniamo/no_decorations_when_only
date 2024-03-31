{
  mkShell,
  statix,
  deadnix,
  rust-analyzer,
  clippy,
  rustfmt,
  no_decorations_when_only,
}:
mkShell {
  inputsFrom = [no_decorations_when_only];
  packages = [
    # Nix
    statix
    deadnix

    # Develop
    rust-analyzer
    clippy
    rustfmt
  ];
}
