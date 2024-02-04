{
  description = "petclinic, just a toy project";

  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          # TODO: Generate a user-friendly version number.
          # version = builtins.substring 0 8 inputs.self.lastModifiedDate;

          pkgs = import nixpkgs {
            inherit system;
            overlays = [ cargo2nix.overlays.default ];
          };

          rustPkgs = pkgs.rustBuilder.makePackageSet {
            rustVersion = "1.73.0";
            packageFun = import ./Cargo.nix;

            packageOverrides = pkgs: pkgs.rustBuilder.overrides.all ++ [
              (pkgs.rustBuilder.rustLib.makeOverride {
                name = "petclinic";
                overrideAttrs = drv: {
                  propagatedNativeBuildInputs = drv.propagatedNativeBuildInputs or [ ] ++
                  [ pkgs.pkg-config pkgs.sqlite pkgs.diesel-cli pkgs.redis ];
                };
              })
            ];

            extraRustComponents = [ "rustfmt" "clippy" ];
          };

          workspaceShell = rustPkgs.workspaceShell {
            packages = with pkgs; [ pkg-config sqlite diesel-cli redis curl fzf rust-analyzer ];
            shellHook = ''
              echo "Welcome to `cargo --version`"
            '';
          };

        in
        rec {
          devShells = {
            default = workspaceShell;
          };

          packages = {
            petclinic = (rustPkgs.workspace.petclinic { }).bin;
            default = packages.petclinic;
          };

        }) // {
      nixosModules.default = { config, lib, pkgs, ... }:
        with lib;
        let
          cfg = config.services.petclinic;

          templates = ./templates;
          static = ./static;
          migrations = ./migrations;

          diesel_config = readFile ./diesel.toml;
        in
        {
          options.services.petclinic = {
            enable = mkEnableOption "Enable the petclinic webside service";

            petclinic-service-port = mkOption {
              type = types.port;
              default = 3000;
              description = "port to listen on";
            };

            petclinic-db-path = mkOption {
              type = types.str;
              default = "/var/lib/petclinic/db.sqlite";
              description = "petclinic sqlitedb";
            };

            redis-server-name = mkOption {
              type = types.str;
              default = "localhost";
              description = "petclinic redis server name";
            };

            tera-templates = mkOption {
              type = types.str;
              default = ''"/etc/petclinic/templates/**/*"'';
              description = "tera templates dir";
            };

            package = mkOption {
              type = types.package;
              default = self.packages.${pkgs.system}.default;
              description = "package to use for this service (defaults to the one in the flake)";
            };
          };

          config = mkIf cfg.enable {
            services.redis.servers."petclinic".enable = true;
            services.redis.servers."petclinic".port = 6379;

            environment.etc."petclinic/release.toml".text = ''
              config_name = "release"
              service_port = ${toString cfg.petclinic-service-port}
              tera_templates = ${cfg.tera-templates}

              [database]
              path = "${cfg.petclinic-db-path}"
              connections = 8

              [redis]
              server = "${cfg.redis-server-name}"

              [session]
              timeout = 10800
            '';

            environment.etc."petclinic/templates".source = "${templates}";
            environment.etc."petclinic/migrations".source = "${migrations}";
            environment.etc."petclinic/static".source = "${static}";

            environment.etc."petclinic/diesel.toml".text = "${diesel_config}";

            users.users."petclinic" = {
              name = "petclinic";
              description = "System user for the redis-server instance petclincic ";
              isSystemUser = true;
              group = "petclinic";
            };
            users.groups."petclinic" = {
              name = "petclinic";
            };

            systemd.services.petclinic = {
              description = "petclinic";
              wantedBy = [ "multi-user.target" ];
              after = [ "redis-petclinic.service" ];

              serviceConfig = {
                ExecStartPre = "+" + pkgs.writeShellScript "prep-conf" (
                  ''
                    cd /etc/petclinic
                    mkdir -p /var/lib/petclinic
                    ${pkgs.diesel-cli}/bin/diesel migration run --migration-dir /etc/petclinic/migrations --database-url /var/lib/petclinic/db.sqlite
                    chown -R petclinic  /var/lib/petclinic
                    chgrp -R petclinic  /var/lib/petclinic
                    chmod 0600 -R /var/lib/petclinic/*
                  ''
                );
                User = "petclinic";
                Group = "petclinic";
                ExecStart = "${cfg.package}/bin/petclinic --config-file /etc/petclinic/release.toml";
                Restart = "on-failure";
                RestartSec = "5s";
              };
            };
          };
        };
    }
  ;
}
