{ pkgs ? import (fetchTarball
  "https://github.com/NixOS/nixpkgs/archive/4d2b37a84fad1091b9de401eb450aae66f1a741e.tar.gz")
  { } }:

pkgs.mkShell {
  buildInputs = [
    pkgs.git
    pkgs.pre-commit
    pkgs.nixfmt
    pkgs.cargo
    pkgs.rustc
    pkgs.rustup
    pkgs.pkg-config # required to find alsa in c code
    pkgs.alsa-lib
    pkgs.SDL2
  ];

  # required for codium rust-analyzer to work
  # https://discourse.nixos.org/t/11570
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
