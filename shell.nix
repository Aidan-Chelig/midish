
{ pkgs ? import <nixpkgs> {} }:

let

  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") { };
in

pkgs.mkShell {

  buildInputs = with pkgs; [
    (
      with fenix;
      combine (
        with default; [
          cargo
          clippy-preview
          latest.rust-src
          rust-analyzer
          rust-std
          rustc
          rustfmt-preview
        ]
      )
    )
    cargo-edit
    cargo-watch
    pkg-config
    alsaLib
    jack2

    lld
    clang

    # # bevy-specific deps (from https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)
  ];

}
