{ pkgs ? import <nixpkgs> {} }: let
  # rustfmt unstable
  fenix = import (pkgs.fetchFromGitHub {
    owner = "nix-community";
    repo = "fenix";
    rev = "3776d0e2a30184cc6a0ba20fb86dc6df5b41fccd";
    sha256 = "K8QDx8UgbvGdENuvPvcsCXcd8brd55OkRDFLBT7xUVY=";
  }) {};

  rust-toolchain = fenix.complete.withComponents [
    "cargo"
    "rustc"
    "clippy"
    "rustfmt"
  ];
in pkgs.mkShell {
  buildInputs = with pkgs; [
    just
    hugo
    rust-toolchain
  ];
}
