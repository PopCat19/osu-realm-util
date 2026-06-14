{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      rec {
        formatter = pkgs.nixfmt;

        checks = {
          nixfmt = pkgs.runCommand "nixfmt-check" { nativeBuildInputs = [ pkgs.nixfmt ]; } ''
            nixfmt --check ${./flake.nix}
            touch $out
          '';
          clippy = pkgs.stdenv.mkDerivation {
            name = "clippy-check";
            src = self;
            nativeBuildInputs = with pkgs; [
              rustc
              cargo
              clippy
              rustfmt
            ];
            buildPhase = ''
              cargo clippy --workspace --all-targets -- -D warnings
              cargo fmt --check
            '';
            installPhase = "touch $out";
          };
        };

        devShells.default = pkgs.mkShell {
          name = "osu-realm-util";
          packages = with pkgs; [
            rustc
            cargo
            rust-analyzer
            rustfmt
            clippy
            nixfmt
          ];
          RUST_BACKTRACE = "1";
          shellHook = ''
            echo "🦀 osu-realm-util devshell"
            echo "  cargo build               -- build"
            echo "  cargo run                  -- test with default client.realm"
            echo "  cargo run ~/path/to/client.realm"
            echo "  cargo clippy -- -D warnings  -- lint"
            echo "  cargo fmt -- --check          -- check formatting"
            echo "  nix fmt                      -- format .nix files"
          '';
        };
      }
    );
}
