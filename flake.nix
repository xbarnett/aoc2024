{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable-small";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ fenix.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };
        llvm = pkgs.llvmPackages_19;
        rust = with fenix.packages.${system}; combine [
          stable.cargo
          stable.rustc
          targets.x86_64-unknown-linux-musl.stable.rust-std
        ];
      in {
        devShell = pkgs.mkShellNoCC {
          buildInputs = [
            llvm.bintools
            rust
          ];
        };
      }
    );
}
