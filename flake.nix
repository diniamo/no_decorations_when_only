{
  description = "A Rust program that utilizies Hyprland's ipc to remove decorations when there is only one window in the workspace";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux"];

      perSystem = {
        pkgs,
        system,
        ...
      }: let
        inherit (pkgs) callPackage;

        no_decorations_when_only = callPackage ./. {};
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

      # flake = {};
    };
}
