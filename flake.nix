{
  description = "A Rust program that utilizies Hyprland's ipc to remove decorations when there is only one window in the workspace";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    systems.url = "github:nix-systems/default-linux";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {systems, flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = import systems;

      perSystem = {pkgs, ...}: let
        no_decorations_when_only = pkgs.callPackage ./. {};
      in {
        formatter = pkgs.alejandra;

        devShells.default = pkgs.mkShell {
          inputsFrom = [no_decorations_when_only];
          packages = with pkgs; [
            # Nix
            alejandra
            statix
            deadnix

            # Develop
            rust-analyzer
            clippy
            rustfmt
          ];
        };

        packages = {
          default = no_decorations_when_only;
          inherit no_decorations_when_only;
        };
      };
    };
}
