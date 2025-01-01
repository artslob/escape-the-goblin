{ pkgs ? import
  (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-24.11.tar.gz")
  { } }:

pkgs.mkShell rec {
  nativeBuildInputs = with pkgs; [ pkg-config ];

  buildInputs = with pkgs; [
    git
    pre-commit
    nixfmt-classic
    cargo
    rustc
    rustup
    alsa-lib-with-plugins
    SDL2
    mesa.drivers
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

  # required for codium rust-analyzer to work
  # https://discourse.nixos.org/t/11570
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
