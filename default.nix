{
  rustPlatform,
  lib,
}: let
  inherit (rustPlatform) buildRustPackage;

  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
in
  buildRustPackage rec {
    pname = cargoToml.package.name;
    version = cargoToml.package.version;

    src = builtins.path {
      name = "${pname}-source";
      path = ./.;
    };

    cargoLock = {
      lockFile = ./Cargo.lock;
      # TODO: remove this when hyprland-rs gets re-released
      outputHashes = {
        "hyprland-0.3.13" = "sha256-Nfgz/8KM0LzaW1RjlIuUYO7Oc86rdrUHERqqQ9yIasI=";
      };
    };

    meta = with lib; {
      description = "A Rust program that utilizies Hyprland's ipc to remove decorations when there is only one window in the workspace";
      homepage = "https://github.com/diniamo/no_decorations_when_only";
      license = licenses.mit;
      maintainers = with maintainers; [diniamo];
      platforms = platforms.linux;
      mainProgram = cargoToml.package.name;
    };
  }
