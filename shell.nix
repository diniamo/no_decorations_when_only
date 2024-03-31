{
  mkShell,
  statix,
  deadnix,
  rust-analyzer,
  clippy,
  rustfmt,
  noDecorationsWhenOnly,
}:
mkShell {
  inputsFrom = [noDecorationsWhenOnly];
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
