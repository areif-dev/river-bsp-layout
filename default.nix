{ fetchFromGitHub, rustPlatform, lib }:
rustPlatform.buildRustPackage {
  pname = "river-bsp-layout";
  version = "2.1.0";

  src = fetchFromGitHub {
    owner = "areif-dev";
    repo = "river-bsp-layout";
    rev = "f27d1d0d492c7e0022695e2a7a0fdf9343877d3d";
    sha256 = "xsH5dyBx7UiIuttBkixjP+SXelyUI9feVncp574G8KU=";
  };
  cargoLock.lockFile = ./Cargo.lock;
}
