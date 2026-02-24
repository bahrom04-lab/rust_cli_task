{pkgs}: let
  # Helpful nix function
  getLibFolder = pkg: "${pkg}/lib";
in
  pkgs.mkShell rec {
    nativeBuildInputs = with pkgs; [
      nixd
      statix
      deadnix
      rustc
      rust-analyzer
      rustfmt
      cargo
      clippy
    ];

  # Set Environment Variables
  RUST_BACKTRACE = "full";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
