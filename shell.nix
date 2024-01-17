let
  rust_overlay = import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };

  rust = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rustfmt" "clippy" ];
    targets = [
      "x86_64-unknown-linux-musl" # used for the backend
      "wasm32-unknown-unknown"    # used for the frontend
    ];
  };
  myRustPlatform = pkgs.makeRustPlatform {
    cargo = pkgs.rust-bin.stable.latest.minimal;
    rustc = pkgs.rust-bin.stable.latest.minimal;
  };
  mytrunk = pkgs.callPackage ./trunk/default.nix { rustPlatform=myRustPlatform; };
in
  with pkgs;
  mkShell {
    buildInputs = [
      mytrunk
      rust
      cargo-zigbuild           # required for static musl builds
      git
      pkg-config
      just                     # task runner
      nodePackages.tailwindcss # build CSS files
      nodejs                   # required to install tailwind plugins
    ];
}
