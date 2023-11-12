{
  description = "Enumorph";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs@{ self, nixpkgs, treefmt-nix, rust-overlay, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      imports = [
        treefmt-nix.flakeModule
      ];

      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          dbg = value:
            builtins.trace (pkgs.lib.generators.toPretty { } value) value;

          nightlyVersion = "2023-11-02";

          crane = rec {
            lib = self.inputs.crane.lib.${system};
            stable = lib.overrideToolchain rust-stable;
          };

          rust-stable = inputs'.rust-overlay.packages.rust.override {
            extensions = [ "rust-src" "rust-analyzer" "clippy" ];
          };

          cargoBuild = crane.stable.buildPackage {
            src = ./.;
            cargoBuildCommand = "cargo build --release";
            buildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [ Security ]);
          };
        in
        {
          packages = {
            default = cargoBuild;
          };
          checks = {
            clippy = crane.stable.cargoClippy {
              src = ./.;
              cargoExtraArgs = "--workspace";
              cargoClippyExtraArgs = "-- -Dwarnings";
              cargoArtifacts = cargoBuild;
            };
            tests = crane.stable.cargoTest {
              src = ./.;
              cargoExtraArgs = "--workspace";
              cargoArtifacts = cargoBuild;
            };
          };
          devShells = {
            default = pkgs.mkShell {
              buildInputs = [ rust-stable pkgs.rnix-lsp ]
                ++ (pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [ Security ]));
            };
          };

          treefmt =
            {
              projectRootFile = "flake.nix";
              programs = {
                nixpkgs-fmt.enable = true;
                rustfmt = {
                  enable = true;
                  package = inputs'.rust-overlay.packages."rust-nightly_${nightlyVersion}";
                };
              };
            };
        };
    };
}
