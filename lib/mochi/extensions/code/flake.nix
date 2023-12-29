{
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixos-23.11;
    flake-utils.url = github:numtide/flake-utils;
  };
  outputs = {
    self,
    flake-utils,
    nixpkgs,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };
      buildNodeJs = pkgs.callPackage "${nixpkgs}/pkgs/development/web/nodejs/nodejs.nix" {
        python = pkgs.python3;
      };
      nodejs = buildNodeJs {
        enableNpm = true;
        version = "20.5.1";
        sha256 = "sha256-Q5xxqi84woYWV7+lOOmRkaVxJYBmy/1FSFhgScgTQZA=";
      };
    in rec {
      devShells.default = pkgs.mkShell {
        inputsFrom = [nodejs];
      };
    });
}
