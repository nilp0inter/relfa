{ inputs, ... }:
{
  flake.homeManagerModules.relfa =
    {
      config,
      lib,
      pkgs,
      ...
    }:
    let
      cfg = config.programs.relfa;
    in
    {
      options.programs.relfa = {
        enable = lib.mkEnableOption "relfa";

        package = lib.mkOption {
          type = lib.types.package;
          default = inputs.self.packages.${pkgs.system}.relfa;
          defaultText = lib.literalExpression "inputs.self.packages.\${pkgs.system}.relfa";
          description = "The relfa package to use.";
        };

        settings = lib.mkOption {
          type = lib.types.submodule {
            freeformType = (pkgs.formats.toml { }).type;
            options = {
              inbox = lib.mkOption {
                type = lib.types.str;
                default = "${config.home.homeDirectory}/Inbox";
                description = "Path to the inbox directory.";
              };

              graveyard = lib.mkOption {
                type = lib.types.str;
                default = "${config.home.homeDirectory}/Graveyard";
                description = "Path to the graveyard directory.";
              };

              age_threshold_days = lib.mkOption {
                type = lib.types.ints.positive;
                default = 3;
                description = "Number of days after which files are considered stale.";
              };

              auto_archive_threshold_days = lib.mkOption {
                type = lib.types.ints.positive;
                default = 7;
                description = "Number of days after which files are automatically archived.";
              };

              notification = lib.mkOption {
                type = lib.types.enum [
                  "cli"
                  "desktop"
                ];
                default = "cli";
                description = "Notification type.";
              };

              pager = lib.mkOption {
                type = lib.types.str;
                default = "less";
                description = "Pager command for viewing files.";
              };

              path_format = lib.mkOption {
                type = lib.types.submodule {
                  options = {
                    created_subdir = lib.mkOption {
                      type = lib.types.submodule {
                        options = {
                          type = lib.mkOption {
                            type = lib.types.enum [ "original" "symlink" "nothing" ];
                            default = "original";
                            description = "Type of subdirectory for created files.";
                          };
                          name = lib.mkOption {
                            type = lib.types.str;
                            default = "created";
                            description = "Name of the created subdirectory.";
                          };
                          target = lib.mkOption {
                            type = lib.types.nullOr lib.types.str;
                            default = null;
                            description = "Target for symlink type (only used when type is 'symlink').";
                          };
                        };
                      };
                      default = {
                        type = "original";
                        name = "created";
                      };
                      description = "Configuration for created files subdirectory.";
                    };

                    modified_subdir = lib.mkOption {
                      type = lib.types.submodule {
                        options = {
                          type = lib.mkOption {
                            type = lib.types.enum [ "original" "symlink" "nothing" ];
                            default = "symlink";
                            description = "Type of subdirectory for modified files.";
                          };
                          name = lib.mkOption {
                            type = lib.types.str;
                            default = "modified";
                            description = "Name of the modified subdirectory.";
                          };
                          target = lib.mkOption {
                            type = lib.types.nullOr lib.types.str;
                            default = "created";
                            description = "Target for symlink type (only used when type is 'symlink').";
                          };
                        };
                      };
                      default = {
                        type = "symlink";
                        name = "modified";
                        target = "created";
                      };
                      description = "Configuration for modified files subdirectory.";
                    };

                    archived_subdir = lib.mkOption {
                      type = lib.types.submodule {
                        options = {
                          type = lib.mkOption {
                            type = lib.types.enum [ "original" "symlink" "nothing" ];
                            default = "symlink";
                            description = "Type of subdirectory for archived files.";
                          };
                          name = lib.mkOption {
                            type = lib.types.str;
                            default = "archived";
                            description = "Name of the archived subdirectory.";
                          };
                          target = lib.mkOption {
                            type = lib.types.nullOr lib.types.str;
                            default = "created";
                            description = "Target for symlink type (only used when type is 'symlink').";
                          };
                        };
                      };
                      default = {
                        type = "symlink";
                        name = "archived";
                        target = "created";
                      };
                      description = "Configuration for archived files subdirectory.";
                    };

                    date_format = lib.mkOption {
                      type = lib.types.str;
                      default = "{hostname}/{year}/{month:02}/{day:02}";
                      description = "Date format string for organizing archived files.";
                    };
                  };
                };
                default = {
                  created_subdir = {
                    type = "original";
                    name = "created";
                  };
                  modified_subdir = {
                    type = "symlink";
                    name = "modified";
                    target = "created";
                  };
                  archived_subdir = {
                    type = "symlink";
                    name = "archived";
                    target = "created";
                  };
                  date_format = "{hostname}/{year}/{month:02}/{day:02}";
                };
                description = "Path format configuration for organizing archived files.";
              };
            };
          };
          default = { };
          description = "Configuration written to {file}`$XDG_CONFIG_HOME/relfa/config.toml`.";
        };

        timer = lib.mkOption {
          type = lib.types.submodule {
            options = {
              enable = lib.mkEnableOption "periodic relfa execution via systemd timer";

              frequency = lib.mkOption {
                type = lib.types.str;
                default = "daily";
                example = "*:0/30";
                description = ''
                  How often to run relfa. This value is passed to the systemd timer's OnCalendar option.
                  See systemd.time(7) for more information about the format.
                '';
              };

              command = lib.mkOption {
                type = lib.types.enum [
                  "scan"
                  "archive"
                  "scan-then-archive"
                ];
                default = "scan";
                description = ''
                  Which relfa command to run periodically:
                  - "scan": Only scan for stale files and show notifications
                  - "archive": Auto-archive files exceeding auto-archive threshold
                  - "scan-then-archive": First scan, then auto-archive eligible files
                '';
              };

              randomizedDelay = lib.mkOption {
                type = lib.types.str;
                default = "1h";
                example = "30m";
                description = ''
                  Add a randomized delay before execution to avoid all machines running at the same time.
                  This value is passed to the systemd timer's RandomizedDelaySec option.
                '';
              };
            };
          };
          default = { };
          description = "Systemd timer configuration for periodic relfa execution.";
        };
      };

      config = lib.mkIf cfg.enable {
        home.packages = [ cfg.package ];

        xdg.configFile."relfa/config.toml" = lib.mkIf (cfg.settings != { }) {
          source = (pkgs.formats.toml { }).generate "relfa-config" cfg.settings;
        };

        systemd.user.services.relfa = lib.mkIf cfg.timer.enable {
          Unit = {
            Description = "Relfa digital file archiver";
            After = [ "graphical-session.target" ];
          };

          Service = {
            Type = "oneshot";
            ExecStart =
              let
                relfaCommand =
                  if cfg.timer.command == "scan" then
                    "${cfg.package}/bin/relfa scan"
                  else if cfg.timer.command == "archive" then
                    "${cfg.package}/bin/relfa archive"
                  # scan-then-archive
                  else
                    "${pkgs.bash}/bin/bash -c '${cfg.package}/bin/relfa scan && ${cfg.package}/bin/relfa archive'";
              in
              relfaCommand;
          };

          Install = {
            WantedBy = [ "default.target" ];
          };
        };

        systemd.user.timers.relfa = lib.mkIf cfg.timer.enable {
          Unit = {
            Description = "Run relfa periodically";
            Requires = [ "relfa.service" ];
          };

          Timer = {
            OnCalendar = cfg.timer.frequency;
            RandomizedDelaySec = cfg.timer.randomizedDelay;
            Persistent = true;
          };

          Install = {
            WantedBy = [ "timers.target" ];
          };
        };
      };
    };
}
