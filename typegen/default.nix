{
  nix-gitignore,
  rustPlatform,
}: rustPlatform.buildRustPackage {
  pname = "quickshell-docs-typegen";
  version = "0.1.0";

  src = nix-gitignore.gitignoreSource [] ./.;
  cargoHash = "sha256-vLj/EKfBzlfRdmVr114evJS+Owzz4PdARNGBE3aPUo4=";
}
