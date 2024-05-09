{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  # this way you could also load rustup with the nix-shell as well... rather
  # than installing it globally.
  buildInputs = with pkgs; [
    cargo
  ];
}
