{ lib, rustPlatform, fetchFromGitHub, pkg-config, openssl, libiconv }:

rustPlatform.buildRustPackage rec {
  pname = "trunk";
  version = "0.18.2";

  src = fetchFromGitHub {
    owner = "trunk-rs";
    repo = "trunk";
    rev = "v${version}";
    sha256 = "sha256-ncN2vXoc6D3OD1bUEe4R4NyxY1+ick709PE+36FXt1k=";
  };

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];

  # requires network
  checkFlags = [ "--skip=tools::tests::download_and_install_binaries" ];

  cargoSha256 = "sha256-wc1uZHkDqKIoEZikP3bH/wBj9JXXFZaIf6r+9f9RxYc=";

  postConfigure = ''
    cargo metadata --offline
  '';

  meta = with lib; {
    homepage = "https://github.com/trunk-rs/trunk";
    description = "Build, bundle & ship your Rust WASM application to the web";
    maintainers = with maintainers; [ freezeboy flosse ];
    license = with licenses; [ asl20 ];
  };
}
