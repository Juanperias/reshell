{
  description = "Retro edit flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";

    fenix.url = "github:nix-community/fenix";
  };

  outputs = { nixpkgs, flake-utils, fenix, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
	crane = inputs.crane.mkLib pkgs;

	toolchain = fenix.packages.${system}.fromToolchainFile {
	    file = ./rust-toolchain.toml;
	    sha256 = "sha256-WGTJJbpV6WEv0VHPBqSIqWLCxzHivFNu0okQ2f9LrWU=";
	};

        craneLib = crane.overrideToolchain toolchain;
      in
      {
        devShells.default = craneLib.devShell {
          packages = with pkgs; [
              just
	      objconv
	      toolchain
	      dosbox
	      djgpp
          ];
        };
      });
}
