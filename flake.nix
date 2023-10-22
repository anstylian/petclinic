{
  description = "petclinic";

  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.73.0";
          packageFun = import ./Cargo.nix;
        };

        workspaceShell = rustPkgs.workspaceShell {
          packages = with pkgs; [ pkg-config sqlite diesel-cli redis curl fzf rust-analyzer ];
        };

      in rec {
        devShells = {
          default = workspaceShell;
        };

        packages = {
          petclinic = (rustPkgs.workspace.petclinic {});
          default = packages.petclinic;
        };
      }
    );
}
