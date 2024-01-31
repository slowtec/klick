# please read flake introduction here:
# https://fasterthanli.me/series/building-a-rust-service-with-nix/part-10#a-flake-with-a-dev-shell
{
  description = "The klick project flake";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs =
  { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          rust = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rustfmt" "clippy" ];
            targets = [
              "x86_64-unknown-linux-musl" # used for the backend
              "wasm32-unknown-unknown"    # used for the frontend
            ];
          };
        in
        with pkgs;
        rec {
          packages.mytrunk = pkgs.callPackage ./trunk/default.nix {};
          devShells.default = mkShell {
            buildInputs = [
              rust
              cargo-zigbuild           # required for static musl builds
              packages.mytrunk
              git
              tig
              pkg-config
              just                     # task runner
              nodePackages.tailwindcss # build CSS files
              nodejs                   # required to install tailwind plugins
              pandoc                   # required to process markdown files
              texliveMedium            # required to generate PDF reports
              librsvg                  # required to render SVG image
            ];
          };
        }
      );
}
