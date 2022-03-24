let
  pkgs = import <nixpkgs> {};

in pkgs.mkShell rec {
  buildInputs = with pkgs; [
    # Toolchain
    cargo rustc gcc

    # Dependencies
    pkg-config gtk3

    # IDE tooling
    rust-analyzer clippy rustfmt
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}
