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
        in
        with pkgs;
        rec {
          packages.mytrunk = pkgs.callPackage ./trunk/default.nix {};
          devShells.default = mkShell {
            buildInputs = [ 
              cargo 
              rustc 
              packages.mytrunk 
              git
              tig
              pkg-config
              just                     # task runner
              nodePackages.tailwindcss # build CSS files
              nodejs                   # required to install tailwind plugins
            ];
          };
        }
      );
}
