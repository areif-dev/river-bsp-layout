{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ 
      (import rust-overlay)
      self.overlays."${system}".default
      ];
    };
  in {
    devShells."${system}".default = pkgs.mkShell {
      buildInputs = with pkgs; [
        rust-bin.stable.latest.default
      ];
    };
    packages."${system}".default = pkgs.river-bsp-layout;

    overlays."${system}".default = final: _prev: {
      river-bsp-layout = final.callPackage ./default.nix { 
        inherit (pkgs) fetchFromGitHub rustPlatform;
      };
    };
  };
}
