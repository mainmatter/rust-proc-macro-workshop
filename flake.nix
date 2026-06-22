{
  description = "Rust procedural macros workshop";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, flake-utils, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;
      in
      {
        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            rust-analyzer
            workshop-runner
            mdbook
            dprint
            cargo-watch
          ];
        };
      });
}
