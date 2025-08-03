{ inputs, ... }:
{
  imports = [
    inputs.devshell.flakeModule
  ];

  perSystem = { config, self', inputs', pkgs, system, ... }: {
    devshells.default = {
      packages = with pkgs; [
        # Add your development dependencies here
        cargo
        rustc
        rust-analyzer
        clippy
        rustfmt
        clang
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
      ];

      env = [
        {
          name = "RUST_SRC_PATH";
          value = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        }
      ];
    };
  };
}