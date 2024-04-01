# No decorations when only

A Hyprland IPC extension written in Rust to remove window decorations when only one window is visible

There is currently no release cycle, so not using Nix isn't ideal.

## Usage

### Nix

1. Add the flake as an input, eg.:
```nix
no_decorations_when_only = {
    url = "github:diniamo/no_decorations_when_only";
    inputs.nixpkgs.follows = "nixpkgs";
};
```
2. If you use home-manager, you can add the executable directly to exec-once, eg.:
```nix
wayland.windowManager.hyprland.settings.exec-once = {
    (lib.getExe inputs.no_decorations_when_only.packages."x86_64-linux".default)
};
```
Otherwise you will have to install the package, and run it with `exec-once`, eg.:
```nix
environment.systemPackages = [
  inputs.no_decorations_when_only.packages."x86_64-linux".default
]
```

### Linux

1. Clone the repository with `git clone https://github.com/diniamo/no_decorations_when_only.git`
2. Run `cargo build --release` in the project root
3. Run the binary (found at `target/release/no_decorations_when_only`) with `exec-once`
