{
  pkgs,
  lib,
  ...
}:
pkgs.rustPlatform.buildRustPackage {
  name = "bing-wallpaper-server";
  src = lib.cleanSource ./.;
  doCheck = false;
  cargoLock.lockFile = ./Cargo.lock;
  nativeBuildInputs = [pkgs.pkg-config];
  buildInputs = [pkgs.openssl];
}
