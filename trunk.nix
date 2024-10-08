{ lib, stdenv, rustPlatform, fetchFromGitHub, pkg-config, openssl, libiconv,
  CoreServices, Security, SystemConfiguration
}:

rustPlatform.buildRustPackage rec {
  pname = "trunk";
  version = "0.20.3";

  src = fetchFromGitHub {
    owner = "trunk-rs";
    repo = "trunk";
    rev = "v${version}";
    sha256 = "sha256-3p3HllZu69e2ERLoEJwSWL0OXl23lxvIPHV9HK30CqM=";
  };

  nativeBuildInputs = [ pkg-config ];
  buildInputs = if stdenv.isDarwin
    then [ libiconv CoreServices Security SystemConfiguration]
    else [ openssl ];

  # requires network
  checkFlags = [ "--skip=tools::tests::download_and_install_binaries" ];

  cargoHash = "sha256-4b+ASz8uV17Y7gO50YKiu8Zhhq4sL+HJj1WAD7VkEE4=";

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
