{
inputs = {
  nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  systems.url = "github:nix-systems/default";
  devenv.url = "github:cachix/devenv";
};

nixConfig = {
  extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
  extra-substituters = "https://devenv.cachix.org";
};

outputs = { self, nixpkgs, devenv, systems, ... } @ inputs:
  let
    forEachSystem = nixpkgs.lib.genAttrs (import systems);
  in
  {
    packages = forEachSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
        antr = pkgs.callPackage ./default.nix { inherit pkgs; };
      });

    devShells = forEachSystem
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [
              {
                packages = with pkgs; [
                  bats
                ] ++ (if pkgs.stdenv.isDarwin then [
                  darwin.apple_sdk.frameworks.Security
                  darwin.apple_sdk.frameworks.CoreServices
                ] else []);

                languages.rust.enable = true;
              }
            ];
          };
        });
  };
}
