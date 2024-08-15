{ fetchFromGitHub, rustPlatform, lib }:
rustPlatform.buildRustPackage rec {
  pname = "river-bsp-layout";
  version = "2.1.0";

  src = fetchFromGitHub {
    owner = "areif-dev";
    repo = "river-bsp-layout";
    rev = "v${version}";
    sha256 = "sha256-LRVZPAS4V5PtrqyOkKUfrZuwLqPZbLoyjn2DPxCFE2o=";
  };

  cargoLock.lockFile = ./Cargo.lock;
}
