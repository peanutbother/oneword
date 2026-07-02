{
  description = "A highly optimized Nix Flake for a Rust project with dependency caching";

  inputs = {
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {self, nixpkgs, flake-utils, rust-overlay, crane}:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = craneLib.cleanCargoSource (craneLib.path ./.);

        commonArgs = {
          inherit src;
          strictDeps = true;
          
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          #buildInputs = with pkgs; [
          #  # --- Linux Dependencies ---
          #  openssl

          #] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          #  # --- macOS Dependencies (System Frameworks) ---
          #  apple-sdk
          #];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        oneword = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
        # What runs with `nix build`
        packages.default = oneword;

        # Development environment (`nix develop`)
        devShells.default = craneLib.devShell {
          inputsFrom = [ oneword ];

          packages = with pkgs; [
            rustToolchain
            cargo-nextest
            sea-orm-cli
          ];

          # Environment variables helpful for rust tooling
          shellHook = ''
            export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"
          '';
        };
      }
    );
}
