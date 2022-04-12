let
  pkgs = import <nixpkgs> {};

in pkgs.mkShell rec {

  buildInputs = with pkgs; [
    gtk3 dbus
    # Dependencies
    # gtk3 dbus-glib glib dbus zlib libxml2 
    # ( dbus.override { systemd = null; } )
    # ( zlib-ng.override { withZlibCompat = true; } )
    # pkg-config gtk3 glib dbus

  ];

  nativeBuildInputs = with pkgs; [
    # Toolchain
    cargo rustc gcc pkg-config
    # IDE tooling
    rust-analyzer clippy rustfmt
  ];

  hardeningDisable = [ "fortify" ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}
