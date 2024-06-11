{
  pkgs,
  lib,
  ...
}:
pkgs.rustPlatform.buildRustPackage rec {
  name = "bing-wallpaper-server";
  src = lib.cleanSource "./";
  doCheck = false;
  cargoLock.lockFile = "${src}/Cargo.lock";
  nativeBuildInputs = [pkgs.pkg-config];
  buildInputs = [pkgs.openssl];
}
