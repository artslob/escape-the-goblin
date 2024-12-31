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
    # glibc
    # pkg-config # required to find alsa in c code
    # alsa-lib
    # alsa-lib.dev
    # udev.dev
    # alsa-lib.dev
    alsa-lib-with-plugins
    # alsa-oss
    # alsa-utils
    # alsa-tools
    # alsa-plugins
    # alsa-ucm-conf
    # alsa-firmware
    # alsa-topology-conf
    # pulseaudio.dev
    # pavucontrol
    SDL2
    # glxinfo
    # mesa
    mesa.drivers
    # libGL
    # vulkan-tools
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

  # required for codium rust-analyzer to work
  # https://discourse.nixos.org/t/11570
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
