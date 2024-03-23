{
  mkShell,
  statix,
  deadnix,
  rustc,
  cargo,
  clippy,
}:
mkShell {
  packages = [
    # Nix
    statix
    deadnix

    # Rust
    rustc
    cargo
    clippy
  ];
}
