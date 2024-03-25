{
  description = "This rust crate exposes te EFF wordlist";
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, naersk, flake-compat, }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in rec {
        # `nix build`
        packages.eff-wordlist = naersk-lib.buildPackage {
          pname = "eff-wordlist";
          root = ./.;
        };
        defaultPackage = packages.eff-wordlist;

        # `nix develop`
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cacert
            cargo
            cargo-edit
            cargo-mutants
            cargo-nextest
            cargo-outdated
            cargo-release
            cargo-watch
            clippy
            git
            llvmPackages_13.llvm
            nixpkgs-fmt
            openssh
            openssl
            pkg-config
            rustc
            rustfmt
            statix
          ];
          RUST_SRC_PATH =
            "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      });
}
