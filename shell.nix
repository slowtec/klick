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
in
  with pkgs;
  mkShell {
    buildInputs = [
      rust
      git
      pkg-config
      just                     # task runner
      trunk                    # frontend build tool
      nodePackages.tailwindcss # build CSS files
    ];
    SQLITE3_DIR = "${sqlite.dev}";
    SQLITE3_LIB_DIR = "${sqlite.out}/lib";
    SQLITE3_INCLUDE_LIB_DIR = "${sqlite.out}/include";
}
