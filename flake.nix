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

        noDecorationsWhenOnly = callPackage ./. {};
      in {
        formatter = pkgs.alejandra;

        devShells.default = callPackage ./shell.nix {inherit noDecorationsWhenOnly;};

        packages = {
          default = noDecorationsWhenOnly;
          inherit noDecorationsWhenOnly;
        };
      };

      # flake = {};
    };
}
