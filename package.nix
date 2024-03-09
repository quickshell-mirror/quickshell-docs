{ pkgs ? import <nixpkgs> {}, srcpath ? ../src }: pkgs.callPackage ./default.nix { inherit srcpath; }
