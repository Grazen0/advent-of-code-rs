{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    git
    rustc
    cargo
    pkg-config
    openssl
  ];
}
