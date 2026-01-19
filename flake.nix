{
  description = "brepimport.rs: A Rust library for importing BREP objects.";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix";
    wasm-server-runner.url = "github:jakobhellermann/wasm-server-runner";
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
    fenix,
    wasm-server-runner,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [fenix.overlays.default];
        };

        buildInputs = with pkgs; [ # runtime dependencies
          pkg-config
          wayland
          wayland-protocols
          libxkbcommon
        ];

        #fenix_complete = pkgs.fenix.complete {
        #  targets = [ "wasm32-unknown-unknown" ];
        #  withComponents = [ "cargo" "clippy" "rust-src" "rustc" "rustfmt" ];
        #};

        # see: https://github.com/nix-community/fenix
        rustToolchain = with pkgs.fenix; combine [
          (with complete; [
            cargo 
            clippy 
            rust-src 
            rustc 
            rustfmt
          ])
          # Add build targets
          targets.wasm32-unknown-unknown.latest.rust-std
        ];

        naersk' = pkgs.callPackage naersk {};
      in rec {
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          inherit buildInputs;
        };

        devShell = pkgs.mkShell {
          inherit buildInputs;
          nativeBuildInputs = (with pkgs; [
            alejandra
            rust-analyzer-nightly
            wasm-tools
            wasm-pack
            http-server
          ]) ++ [
            rustToolchain
            wasm-server-runner.packages.${system}.default
          ] ;
          shellHook = ''
            export RUST_BACKTRACE=1
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
              pkgs.wayland
              pkgs.libxkbcommon
              pkgs.udev
            ]}"
            export rust_toolchain="${rustToolchain}"
          '';
        };
      }
    );
}
