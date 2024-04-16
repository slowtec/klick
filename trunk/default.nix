{ lib, stdenv, rustPlatform, fetchFromGitHub, pkg-config, openssl, libiconv,
  CoreServices, Security, SystemConfiguration
}:

rustPlatform.buildRustPackage rec {
  pname = "trunk";
  version = "0.19.2";

  src = fetchFromGitHub {
    owner = "trunk-rs";
    repo = "trunk";
    rev = "v${version}";
    sha256 = "sha256-iVwnuTCOgUbmU0jdiWmtzgd4nulVY17IMqlh3bh3gqE=";
  };

  nativeBuildInputs = [ pkg-config ];
  buildInputs = if stdenv.isDarwin
    then [ libiconv CoreServices Security SystemConfiguration]
    else [ openssl ];

  # requires network
  checkFlags = [ "--skip=tools::tests::download_and_install_binaries" ];

  cargoSha256 = "sha256-rsjM12451mMrBkNNUmvqHOn4ep7epfESCAYu04o28+4=";

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
