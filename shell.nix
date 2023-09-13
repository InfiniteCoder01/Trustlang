{ pkgs ? import <nixpkgs> { } }:
with pkgs; mkShell {
  inputsFrom = [];
  buildInputs = [ rustup ];
}
