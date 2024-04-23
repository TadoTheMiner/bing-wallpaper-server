This simple program runs a server that provides Bing images of the day at a single URL, so you can use it as wallpaper (I use it in the NightTab Firefox extension).
You can specify the port where it gets ran (default is 8080) as an argument, and the path to image cache.
# Instalation
1. Run ```cargo install --git https://github.com/TadoTheMiner/bing-wallpaper-server/```
## Nixos 
The nixos package specification is 
```nix
{ pkgs, fetchFromGitHub, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "bing-wallpaper-server";
  version = "35d4ed1";

  src = fetchFromGitHub {
    owner = "TadoTheMiner";
    repo = pname;
    rev = "35d4ed15217708b966b06256b15219da4bc41fa2";
    hash = "sha256-eB9BtKKMZnsrqySQAYVW9pOAGGf95M1jvySsIWeuois=";
  };
  doCheck = false;
  cargoHash = "sha256-9haW0Ko6PF/TGuXYhWH07bE/J3Fd/dR3ck/Im8Mo5dg=";
  nativeBuildInputs = with pkgs; [ pkg-config ];
  buildInputs = with pkgs; [ openssl ];
}

```
Replace version with the first 7 letters in the latest commit hash, rev with the latest commit has, and hash and cargoHash with whathever hash you will get.  
2. Add it to the end of your shell profile so it autostarts (make sure you run it in the background otherwise your profile will never stop executing)
3. You should have your image in http://127.0.0.1:8080/bing-daily-wallpaper
