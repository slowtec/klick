{ lib, rustPlatform, fetchFromGitHub, pkg-config, openssl, libiconv }:

rustPlatform.buildRustPackage rec {
  pname = "trunk";
  version = "0.18.3";

  src = fetchFromGitHub {
    owner = "trunk-rs";
    repo = "trunk";
    rev = "v${version}";
    sha256 = "sha256-R7i2tY8wd7Jhyx+zs+OqkZ+K+d/triBRqaAsATtCM+o=";
  };

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];

  # requires network
  checkFlags = [ "--skip=tools::tests::download_and_install_binaries" ];

  cargoSha256 = "sha256-72EOpxcQ06uQG9DB/Ef6rcs/ce1jlS+Fj663qFRYQsA=";

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
