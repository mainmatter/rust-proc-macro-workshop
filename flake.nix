{
  description = "Rust procedural macros workshop";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = (crane.mkLib pkgs).overrideToolchain
          fenix.packages.${system}.stable.toolchain;
      in
      {
        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            workshop-runner
            mdbook
            dprint
            cargo-watch
            cargo-expand
          ];
        };
      });
}
