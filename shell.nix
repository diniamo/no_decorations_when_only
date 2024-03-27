{
  mkShell,
  statix,
  deadnix,
  rust-analyzer,
  clippy,
  rustfmt,
  rustc,
  cargo,
}:
mkShell {
  packages = [
    # Nix
    statix
    deadnix

    # Develop
    rust-analyzer
    clippy
    rustfmt

# Build
    rustc
    cargo
  ];
}
