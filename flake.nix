{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {inherit system overlays;};
        rustVersion = {
          dev = pkgs.rust-bin.stable.latest.default.override {extensions = ["rust-analyzer" "rust-src"];};
          build = pkgs.rust-bin.stable.latest.minimal;
        };
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion.build;
          rustc = rustVersion.build;
        };
      in {
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              openssl
              pkg-config
              rustVersion.dev
            ];
          };

        packages.default = rustPlatform.buildRustPackage {
          name = "database-rs";
          version = "0.1.0";

          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;
        };
      }
    );
}
