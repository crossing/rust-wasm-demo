{ pkgs ? import <nixpkgs> { }}:
pkgs.mkShell {
  name = "rust-wasm-demo";
  buildInputs = with pkgs; [
    wasm-pack
    wabt
    rustup
    rust-analyzer

    cargo
    cargo-watch
    cargo-generate

    nodejs
  ];
}
