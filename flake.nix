{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";

    quickshell = {
      url = "git+https://git.outfoxxed.me/quickshell/quickshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, quickshell }: let
    forEachSystem = fn: nixpkgs.lib.genAttrs
      [ "x86_64-linux" "aarch64-linux" ]
      (system: fn system nixpkgs.legacyPackages.${system});
  in {
    packages = forEachSystem (system: pkgs: rec {
      quickshell-docs = import ./package.nix {
        inherit pkgs;
        srcpath = "${quickshell}/src";
      };

      quickshell-types = pkgs.callPackage ./types.nix {
        srcpath = "${quickshell}/src";
      };

      default = quickshell-docs;
    });
  };
}
