{ rustPlatform, version ? "git" }:
rustPlatform.buildRustPackage {
  pname = "river-bsp-layout";
  inherit version;

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
