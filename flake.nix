{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      crane,
      flake-utils,
      ...
    }:
    let
      # these platforms are by far the most common so we use them to build
      # and test the lib in CI, but it should work on other platforms too.
      supportedSystems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];
    in
    flake-utils.lib.eachSystem supportedSystems (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default;
        craneLib = (crane.mkLib nixpkgs.legacyPackages.${system}).overrideToolchain rustToolchain;

        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;

          nativeBuildInputs = with pkgs; [
            pkg-config
            # This switches rustfmt to the nightly channel.
            rust-bin.nightly.latest.rustfmt
          ];

          cargoExtraArgs = "--all-features";
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        maybe-multiple = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
        maybe-multiple-fmt = craneLib.cargoFmt (commonArgs // { inherit cargoArtifacts; });
        maybe-multiple-clippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets --no-deps -- --deny warnings --deny clippy::all";
          }
        );
        maybe-multiple-test = craneLib.cargoTest (commonArgs // { inherit cargoArtifacts; });
      in
      {
        packages.default = maybe-multiple;

        checks = {
          inherit
            maybe-multiple
            maybe-multiple-fmt
            maybe-multiple-clippy
            maybe-multiple-test
            ;
        };

        devShells.default = craneLib.devShell { checks = self.checks.${system}; };
      }
    );
}
