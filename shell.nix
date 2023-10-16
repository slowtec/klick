let
  rust_overlay = import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };

  rust = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rustfmt" "clippy" ];
    targets = [
      "x86_64-unknown-linux-musl"
      "wasm32-unknown-unknown"
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
      git
      pkg-config
      just                     # task runner
      nodePackages.tailwindcss # build CSS files
      nodejs                   # required to install tailwind plugins
    ];
    SQLITE3_DIR = "${sqlite.dev}";
    SQLITE3_LIB_DIR = "${sqlite.out}/lib";
    SQLITE3_INCLUDE_LIB_DIR = "${sqlite.out}/include";
}
