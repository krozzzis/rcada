{
  description = "A Rust workspace with an Iced GUI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # Apply the overlay to get recent Rust toolchains
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Define the Rust toolchain. You can change 'stable' to 'nightly' or a specific version.
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ]; # 'rust-src' is needed for rust-analyzer
        };

        # System dependencies required for Iced (wgpu backend)
        iced-deps = with pkgs; [
          # Core build tools
          pkg-config
          stdenv.cc # Provides the 'cc' linker
        ];
      in
      {
        # Development environment activated with `nix develop`
        devShells.default = pkgs.mkShell {
          buildInputs = [ rustToolchain ] ++ iced-deps;

          # Environment variables for rust-analyzer
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

          # This is crucial for runtime linking. It tells the application where to find
          # shared libraries like libwayland-client.so when you run `cargo run`.
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath iced-deps;
        };

        # App to run with `nix run`
        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };

        # Formatter for Nix code, run with `nix fmt`
        formatter = pkgs.nixpkgs-fmt;
      });
}
