This simple program runs a server that provides Bing images of the day at a single URL, so you can use it as wallpaper (I use it in the NightTab Firefox extension).
You can specify the port where it gets ran (default is 8080) as an argument, and the path to image cache.
# Instalation
1. Run ```cargo install --git https://github.com/TadoTheMiner/bing-wallpaper-server/```
## Nixos 
The nixos package specification is 
```nix
{ pkgs, ... }:
(pkgs.makeRustPlatform {
  cargo = pkgs.rust-bin.stable.latest.default;
  rustc = pkgs.rust-bin.stable.latest.default;
}).buildRustPackage {
  pname = "bing-wallpaper-server";
  version = "4900c7e";

  src = pkgs.fetchFromGitHub {
    owner = "TadoTheMiner";
    repo = "bing-wallpaper-server";
    rev = "4900c7e5322ed64a81d74bd07b5a55d9dc358eb8";
    hash = "sha256-46GU8olBTFqxNz02KmOmZUmlaDjTXlcyOV0EkU2FjQQ=";
  };
  doCheck = false;
  cargoHash = "sha256-DmtgN5L0lBFvdqZKHxcO0JkeNkBesE3Jo+Xb9+pDRmE=";
  nativeBuildInputs = with pkgs; [ pkg-config ];
  buildInputs = with pkgs; [ openssl ];
}

```
Replace version with the first 7 letters in the latest commit hash, rev with the latest commit has, and hash and cargoHash with whathever hash you will get. Use this overlay https://github.com/oxalica/rust-overlay#classic-nix-overlay  
2. Add it to the end of your shell profile so it autostarts (make sure you run it in the background otherwise your profile will never stop executing)
3. You should have your image in http://127.0.0.1:8080/bing-daily-wallpaper
