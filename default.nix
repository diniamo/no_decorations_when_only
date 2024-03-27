{
  rustPlatform,
  lib,
}: let
  inherit (rustPlatform) buildRustPackage;

  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
in
  buildRustPackage {
    pname = cargoToml.package.name;
    version = cargoToml.package.version;

    src = ./.;

    cargoLock.lockFile = ./Cargo.lock;

    meta = with lib; {
      description = "A Rust program that utilizies Hyprland's ipc to remove decorations when there is only one window in the workspace";
      homepage = "https://github.com/diniamo/no_decorations_when_only";
      license = licenses.mit;
      maintainers = with maintainers; [diniamo];
      platforms = platforms.linux;
      mainProgram = cargoToml.package.name;
    };
  }
