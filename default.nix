{ rustPlatform, version ? "git", lib }:
rustPlatform.buildRustPackage {
  pname = "river-bsp-layout";
  inherit version;

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;
}
