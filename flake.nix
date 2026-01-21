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

        nativeBuildInputs = (with pkgs; [ # build-time dependencies
          pkg-config
          vulkan-headers
        ]) ++ [ # Rust toolchain
          rustToolchain
          wasm-server-runner.packages.${system}.default
        ];

        buildInputs = with pkgs; [ # runtime dependencies
          mesa
          libdrm
          udev
          # Vulkan
          vulkan-loader
          vulkan-validation-layers
          vulkan-extension-layer
          # Wayland
          wayland
          wayland-protocols
          # X11
          xorg.libX11
          xorg.libXrandr
          xorg.libXcursor
          xorg.libXinerama
          xorg.libXext
          xorg.libXi
          libxkbcommon
        ];

        naersk' = pkgs.callPackage naersk {};
      in rec {
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          inherit buildInputs nativeBuildInputs;
        };

        devShell = pkgs.mkShell {
          inherit buildInputs;
          nativeBuildInputs = nativeBuildInputs ++ (with pkgs; [
            alejandra
            http-server
            wasm-tools
            wasm-pack
            vulkan-tools # provides: vkcube vkcubepp vulkaninfo
            # LSP Shiz
            rust-analyzer-nightly
            wgsl-analyzer
          ]);
          # Dev note: `shellHook` should be posix sh compliant
          shellHook = ''
            export TOP="$(realpath ./)"
            export RUST_BACKTRACE=full
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath buildInputs}"
            export rust_toolchain="${rustToolchain}"
            run-wasm() {
              wasm-pack build --target web && http-server
            }
            # Force Winit to use X11
            winit-force-X11() {
              export WINIT_UNIX_BACKEND=x11
              export OLD_WAYLAND_DISPLAY="$WAYLAND_DISPLAY"
              export OLD_XDG_SESSION_TYPE="$XDG_SESSION_TYPE"
              unset WAYLAND_DISPLAY
              unset XDG_SESSION_TYPE
            }
            # Revert winit-force-X11
            winit-unset-X11() {
              unset WINIT_UNIX_BACKEND=x11
              export WAYLAND_DISPLAY="$OLD_WAYLAND_DISPLAY"
              export XDG_SESSION_TYPE="$OLD_XDG_SESSION_TYPE"
            }
          '';
        };
      }
    );
}
