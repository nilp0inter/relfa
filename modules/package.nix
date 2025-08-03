{ inputs, ... }:
{
  perSystem = { config, self', inputs', pkgs, system, ... }: 
  let
    cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
  in
  {
    packages.relfa = pkgs.rustPlatform.buildRustPackage rec {
      pname = "relfa";
      version = cargoToml.package.version;

      src = ../.;

      cargoLock = {
        lockFile = ../Cargo.lock;
      };

      nativeBuildInputs = with pkgs; [
        pkg-config
      ];

      buildInputs = with pkgs; [
        dbus
      ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
        pkgs.darwin.apple_sdk.frameworks.Foundation
        pkgs.darwin.apple_sdk.frameworks.UserNotifications
      ];

      meta = with pkgs.lib; {
        description = cargoToml.package.description;
        homepage = cargoToml.package.homepage;
        license = licenses.mit;
        maintainers = [ ];
        mainProgram = "relfa";
      };
    };

    # Make relfa the default package
    packages.default = config.packages.relfa;
  };
}