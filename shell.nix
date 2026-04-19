{
  pkgs,
}:
let
  overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    # Nix
    nil
    nixfmt

    # Rust
    cargo
    rust-analyzer
    rustfmt
    clippy
    rustc
    rustup

    # Dioxus
    dioxus-cli

    lld
  ];

  RUSTC_VERSION = overrides.toolchain.channel;
  CARGO_BUILD_TARGET = "wasm32-unknown-unknown";

  shellHook = ''
    export PATH="''${CARGO_HOME:-~/.cargo}/bin":"$PATH"
    export PATH="''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-${pkgs.stdenv.hostPlatform.rust.rustcTarget}/bin":"$PATH"

    rustup target add "wasm32-unknown-unknown"
  '';
}
