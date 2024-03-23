{buildRustPackage}: let
  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
in
  buildRustPackage {
    pname = cargoToml.package.name;
    version = cargoToml.package.version;

    src = ./.;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };
  }
