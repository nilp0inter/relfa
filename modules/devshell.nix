{ inputs, ... }:
{
  imports = [
    inputs.devshell.flakeModule
  ];

  perSystem =
    { config, pkgs, ... }:
    {
      devshells.default = {
        packages =
          with pkgs;
          [
            # Add your development dependencies here
            cargo
            rustc
            rust-analyzer
            clippy
            rustfmt
            clang
          ]
          ++ [
            config.treefmt.build.wrapper
          ];

        commands = [
          {
            help = "Run cargo build";
            name = "build";
            command = "cargo build";
          }
          {
            help = "Run cargo test";
            name = "test";
            command = "cargo test";
          }
          {
            help = "Run cargo check";
            name = "check";
            command = "cargo check";
          }
          {
            help = "Format all files";
            name = "fmt";
            command = "treefmt";
          }
          {
            help = "Install git hooks";
            name = "install-hooks";
            command = "pre-commit install";
          }
        ];

        env = [
          {
            name = "RUST_SRC_PATH";
            value = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          }
        ];

        devshell.startup.pre-commit-install = {
          text = ''
            ${config.pre-commit.installationScript}
            git config --local --unset core.hooksPath || true
            pre-commit install --install-hooks
          '';
        };
      };
    };
}
