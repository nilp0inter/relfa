{ inputs, ... }:
{
  imports = [
    inputs.git-hooks-nix.flakeModule
  ];

  perSystem =
    { pkgs, ... }:
    {
      pre-commit = {
        check.enable = true;

        settings = {
          hooks = {
            # Rust formatting check (matches CI: cargo fmt --all -- --check)
            rustfmt = {
              enable = true;
              packageOverrides.cargo = pkgs.cargo;
              packageOverrides.rustfmt = pkgs.rustfmt;
            };

            # Rust linting (matches CI: cargo clippy --all-targets --all-features -- -D warnings)
            cargo-clippy = {
              enable = true;
              name = "cargo-clippy";
              entry = "${pkgs.cargo}/bin/cargo clippy --all-targets --all-features -- -D warnings";
              language = "system";
              files = "\\.rs$";
              pass_filenames = false;
            };

            # Run cargo test (matches CI: cargo test --verbose)
            cargo-test = {
              enable = true;
              name = "cargo-test";
              entry = "${pkgs.cargo}/bin/cargo test --verbose";
              language = "system";
              files = "\\.(rs|toml)$";
              pass_filenames = false;
            };
          };
        };
      };

      # Pre-commit hooks are automatically installed when entering the devshell
    };
}
