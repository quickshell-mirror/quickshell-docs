{
  nix-gitignore,
  rustPlatform,
}: rustPlatform.buildRustPackage {
  pname = "quickshell-docs-typegen";
  version = "0.1.0";

  src = nix-gitignore.gitignoreSource [] ./.;
  cargoHash = "sha256-bOOYzCLIyze6DbtMDupSuRFgJAKjbcFXrZw7vclorYQ=";
}
