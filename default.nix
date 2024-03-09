{
  stdenv,
  nix-gitignore,
  hugo,
  cargo,
  just,

  callPackage,
  typegen ? (callPackage ./typegen {}),

  srcpath ? ../src,
}: stdenv.mkDerivation {
  name = "quickshell-docs";
  version = "0.1.0";
  src = nix-gitignore.gitignoreSource "/typegen\n" ./.;

  buildInputs = [
    just
    hugo
    typegen
  ];

  buildPhase = ''
    SRC_PATH="${srcpath}" TYPEGEN=typegen just build
  '';

  installPhase = ''
    mkdir -p $out
    cp -r ./public/* $out/
  '';
}
