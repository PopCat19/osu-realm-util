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

        packages = rec {
          default = osu-realm-util;
          osu-realm-util = pkgs.rustPlatform.buildRustPackage {
            pname = "osu-realm-util";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
            checkPhase = ''
              cargo test --workspace -- --skip real_file_parse
            '';
            meta = with pkgs.lib; {
              description = "Read osu! Realm databases and legacy collection.db files";
              license = licenses.mit;
              mainProgram = "osu-realm-util";
            };
          };
        };

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
            echo "  cargo run                  -- list tables from default client.realm"
            echo "  cargo run -- col            -- list collections from default collection.db"
            echo "  cargo run -- realm2col DB   -- export lazer collections"
            echo "  cargo run -- merge DB       -- merge lazer into existing collection.db"
            echo "  cargo clippy -- -D warnings -- lint"
            echo "  cargo fmt -- --check        -- check formatting"
            echo "  nix fmt                     -- format .nix files"
          '';
        };
      }
    );
}
