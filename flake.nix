{
  description = "Nix flake template for Rust projects";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    _inputs@{ self, nixpkgs, ... }:
    let
      inherit (nixpkgs) lib;

      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = lib.genAttrs supportedSystems;
      pkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in
    {
      packages = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./package.nix { pkgs = pkgsFor.${system}.pkgs; };
      });

      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./shell.nix { pkgs = pkgsFor.${system}.pkgs; };
      });

      formatter = forAllSystems (system: {
        default = pkgsFor.${system}.pkgs.nixfmt-rfc-style;
      });

      # TODO: implement these two
      nixosModules.default = ./module.nix { };
      homeManagerModules.default = ./hm-module.nix { };
    };
}
