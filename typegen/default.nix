{
  nix-gitignore,
  rustPlatform,
}: rustPlatform.buildRustPackage {
  pname = "quickshell-docs-typegen";
  version = "0.1.0";

  src = nix-gitignore.gitignoreSource [] ./.;
  cargoSha256 = "rep68gbnp9uPhzjK7opLg7dh4X2uKNmAPfGUuGjE35w=";
}
