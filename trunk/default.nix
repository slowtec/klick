{ lib, rustPlatform, fetchFromGitHub, pkg-config, openssl, libiconv }:

rustPlatform.buildRustPackage rec {
  pname = "trunk";
  version = "0.17.5";

  src = fetchFromGitHub {
    owner = "thedodd";
    repo = "trunk";
    rev = "v${version}";
    sha256 = "sha256-CRlSHOT4hMblfaTcX9Y2BN52RYjDSLaanoxOMccff40=";#sha256-/XVDjKK1Kv7Hk3RUf4v9PEGwarUGzj3+96mZoCKrUEw=";
  };

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];

  # requires network
  checkFlags = [ "--skip=tools::tests::download_and_install_binaries" ];

  cargoSha256 = "sha256-WZ9EOQ00pOTqlO9G/Q58QMU2BMFfMY6JtTVzmMMKfys=";
  #cargoBuildIgnoreFrozenFlag = true;
  #cargoBuildFlags = [ "--offline" ];

    # Cargo.lock is outdated
  postConfigure = ''
    cargo metadata --offline
  '';

  meta = with lib; {
    homepage = "https://github.com/thedodd/trunk";
    description = "Build, bundle & ship your Rust WASM application to the web";
    maintainers = with maintainers; [ freezeboy ];
    license = with licenses; [ asl20 ];
  };
}
