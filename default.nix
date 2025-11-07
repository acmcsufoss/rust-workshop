with import <nixpkgs> {};
mkShell {
  nativeBuildInputs = [
    clang
    gnumake
    cargo
  ];
}
