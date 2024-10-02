{ pkgs }:

with pkgs;
rustPlatform.buildRustPackage rec {
  name = "antr-${version}";
  version = "2.0.0-beta";
  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  buildInputs = [
    darwin.apple_sdk.frameworks.Security
    darwin.apple_sdk.frameworks.CoreServices
  ];

  meta = with lib; {
    description = "A simple to use and high performance file watcher.";
    homepage = "https://github.com/juanibiapina/antr";
    license = licenses.mit;
  };
}
