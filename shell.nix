{ pkgs ? import <nixpkgs> { }}:
pkgs.mkShell {
  name = "rust-wasm-demo";
  buildInputs = with pkgs; [
    wasm-pack
    rustup
    rust-analyzer

    cargo
    cargo-watch
    cargo-generate

    nodejs
  ];
}
