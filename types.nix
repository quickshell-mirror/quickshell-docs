{
  stdenv,
  nix-gitignore,
  just,

  callPackage,
  typegen ? (callPackage ./typegen {}),
  srcpath ? ../src,
}: stdenv.mkDerivation {
  name = "quickshell-types";
  version = "0.1.0";
  src = nix-gitignore.gitignoreSource "/typegen\n" ./.;

  buildInputs = [ just typegen ];

  buildPhase = ''
    SRC_PATH="${srcpath}" TYPEGEN=typegen just typedocs
  '';

  installPhase = ''
    mv ./data/modules $out
  '';
}
