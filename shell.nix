{ pkgs ? import <nixpkgs> { } }: pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    gcc
    clang_16


    # for building things
    gnumake
  ];
  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
  '';
  packages = (with pkgs; [
    gef
    rust-bindgen
    shellcheck
  ]);
}
