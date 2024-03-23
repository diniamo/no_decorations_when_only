{
  mkShell,
  statix,
  deadnix,
  rust-analyzer,
  clippy,
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

# Build
    rustc
    cargo
  ];
}
