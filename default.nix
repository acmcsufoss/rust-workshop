with import <nixpkgs> {};
mkShell {
  nativeBuildInputs = [
    clang
    cargo
  ];
}
