{
  description = "Zed Shell Slash Command Extension";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-wasi" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            openssl
            cargo-wasi
            wasmtime
            wasm-pack
            gnumake
          ];

          shellHook = ''
            echo "Zed Shell Command Extension Development Environment"
            echo "Run 'make' to build the extension"
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "zed-shell-slash-command";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          
          buildPhase = ''
            cargo build --release --target wasm32-wasi
          '';

          installPhase = ''
            mkdir -p $out
            cp target/wasm32-wasi/release/shell_command.wasm $out/
            cp extension.toml $out/
          '';
        };
      });
}