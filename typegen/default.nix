{
  nix-gitignore,
  rustPlatform,
}: rustPlatform.buildRustPackage {
  pname = "quickshell-docs-typegen";
  version = "0.1.0";

  src = nix-gitignore.gitignoreSource [] ./.;
  cargoSha256 = "4bH7E+NpZPFtr//l00tYkHkRRbI3D0TkKas1M+vDWpI=";
}
