# This file is pretty general, and you can adapt it in your project replacing
# only `name` and `description` below.

{
  description = "hector";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, rust-overlay, ... }:
    utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          # Imports
          pkgs = import nixpkgs {
            inherit system overlays;
          };

          # Configuration for the non-Rust dependencies
          buildInputs = with pkgs; [ ];
          nativeBuildInputs = with pkgs; [ rustc cargo pkgconfig ];
        in
        rec {
          # `nix develop`
          devShell = pkgs.mkShell
            {
              buildInputs = buildInputs ++ (with pkgs;
                # Tools you need for development go here.
                [
                  (rust-bin.stable.latest.default.override {
                    extensions = [ "rust-src" ];
                  })
                  nixpkgs-fmt
                  cargo-outdated
                  cargo-edit
                ]);
            };
        }
      );
}

