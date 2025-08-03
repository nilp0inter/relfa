{ inputs, ... }:
{
  imports = [
    inputs.treefmt-nix.flakeModule
  ];

  perSystem = { config, self', inputs', pkgs, system, ... }: {
    treefmt = {
      projectRootFile = ".git/config";
      
      programs = {
        # Rust code formatting
        rustfmt.enable = true;
        
        # TOML formatting (for Cargo.toml)
        taplo.enable = true;
        
        # Nix code formatting
        nixfmt.enable = true;
      };

      settings.global.excludes = [
        "*.1"  # Exclude man pages
        ".git/*"
        "flake.lock"
      ];
    };
  };
}