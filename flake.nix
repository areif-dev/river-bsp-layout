{
  description =
    "A flake that contains the devshell, package, and package overlay of river-bsp-layout";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs@{ self, flake-parts, nixpkgs, rust-overlay, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "i686-linux" "aarch64-linux" ];
      flake = {
        overlays.default = final: prev: {
          river-bsp-layout = final.callPackage ./default.nix {
            inherit (prev) fetchFromGitHub rustPlatform;
          };
        };
      };
      
      perSystem = { config, system, pkgs, ... }: {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import inputs.rust-overlay) self.overlays.default ];
        };
        
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [ rust-bin.stable.latest.default ];
        };
        
        packages.river-bsp-layout = pkgs.river-bsp-layout;
        packages.default = config.packages.river-bsp-layout;

        apps.river-bsp-layout.program = "${config.packages.river-bsp-layout}/bin/river-bsp-layout";
        apps.default.program = config.apps.river-bsp-layout.program;
      };
    };
}
