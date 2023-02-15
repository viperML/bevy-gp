{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    oxalica-rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["aarch64-linux" "x86_64-linux"];

      perSystem = {
        pkgs,
        inputs',
        ...
      }: {
        _module.args.pkgs = inputs'.nixpkgs.legacyPackages.extend inputs.oxalica-rust-overlay.overlays.default;

        devShells.default = with pkgs; let
          toolchain = rust-bin.fromRustupToolchainFile ./toolchain.toml;
        in
          mkShell {
            RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
            packages = [
              toolchain
              trunk
              wasm-bindgen-cli
              binaryen

              # pkg-config
              # wayland
              # libxkbcommon
              # systemd
              # libffi
            ];
          };
      };
    };
}
