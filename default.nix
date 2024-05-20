{ fetchFromGitHub, rustPlatform, lib }:
rustPlatform.buildRustPackage {
  pname = "river-bsp-layout";
  version = "2.0.0";

  src = fetchFromGitHub {
    owner = "areif-dev";
    repo = "river-bsp-layout";
    rev = "dfe84fc12fd6d0e6afb3d28bd94cc26805d3923d";
    hash = "sha256-WgjC+BcyMaToOw+endHccjCHrCyi4NGN+611AI3FhC8=";
  };
  cargoLock.lockFile = ./Cargo.lock;
}
