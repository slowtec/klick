{ lib, rustPlatform, fetchFromGitHub, pkg-config, openssl, libiconv }:

rustPlatform.buildRustPackage rec {
  pname = "trunk";
  version = "0.18.8";

  src = fetchFromGitHub {
    owner = "trunk-rs";
    repo = "trunk";
    rev = "v${version}";
    sha256 = "sha256-cx14IVqsu1SQezs8T1HFZ75+MPWkvf5RcvGCodW5G4A=";
  };

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];

  # requires network
  checkFlags = [ "--skip=tools::tests::download_and_install_binaries" ];

  cargoSha256 = "sha256-70nSHOGUIege1JKIt+WRnAm9VJ5/npxMGG57naTVyEs=";

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
