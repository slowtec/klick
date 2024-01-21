{ lib, rustPlatform, fetchFromGitHub, pkg-config, openssl, libiconv }:

rustPlatform.buildRustPackage rec {
  pname = "trunk";
  version = "0.18.6";

  src = fetchFromGitHub {
    owner = "trunk-rs";
    repo = "trunk";
    rev = "v${version}";
    sha256 = "sha256-cdPRVh+0PGhJO8FB+bYITUFraHVHXOGtB2Q26dT3thk=";
  };

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];

  # requires network
  checkFlags = [ "--skip=tools::tests::download_and_install_binaries" ];

  cargoSha256 = "sha256-HMGFlLxwr1nQxihOPiYRfKfrqStVlII4t7+5t4tQYk4=";

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
