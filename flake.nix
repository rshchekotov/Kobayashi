{
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixos-unstable;
    flake-utils.url = github:numtide/flake-utils;
    rust-overlay = {
      url = github:oxalica/rust-overlay;
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    crane = {
      url = github:ipetkov/crane;
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    crane,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {inherit system overlays;};
        inherit (pkgs) lib;

        rustToolchain = (pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
          extensions = ["rust-src"];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = lib.cleanSourceWith {
          src = ./.;
          filter = path: type:
            (lib.hasSuffix "\.pest" path)
            || (craneLib.filterCargoSources path type);
        };

        nativeBuildInputs = with pkgs; [rustToolchain rust-analyzer pkg-config];
        buildInputs = with pkgs; [];

        commonArgs = {
          inherit src buildInputs nativeBuildInputs;
          strictDeps = true;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        bin = craneLib.buildPackage (commonArgs
          // {
            inherit cargoArtifacts;
          });
      in
        with pkgs; {
          packages = {
            inherit bin;
            default = bin;
          };
          checks = {
            inherit bin;
            crate-fmt = craneLib.cargoFmt {
              inherit src;
            };
            cargo-test = craneLib.cargoNextest (commonArgs
              // {
                inherit cargoArtifacts;
                cargoExtraArgs = "--all-targets";
                partitions = 1;
                partitionType = "count";
              });
          };
          devShells.default = mkShell {
            inputsFrom = [bin];
          };
        }
    );
}
