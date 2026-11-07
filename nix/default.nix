let
  inputs = import ./.tack {};
  inherit
    (inputs)
    nixpkgs
    nix-vessel
    rust-overlay
    ;
  overlays = [(import rust-overlay)];
  pkgs = mk-pkgs builtins.currentSystem;
  mk-pkgs = system: import nixpkgs {inherit overlays system;};
  mk-packages = pkgs: import ./packages {inherit pkgs;};
  mk-shell = pkgs: packages:
    nix-vessel.mk-shell {
      inherit pkgs inputs;
      addons = ["gum" "treefmt"];
      modules = [
        {
          packages = [packages.rust-toolchain];
          inputs-from = [packages.default];
        }
        ./shell
      ];
    };
  out = rec {
    packages = mk-packages pkgs;
    checks = {
      vm-test = import ./tests/vm-test.nix {
        inherit pkgs;
        project-package = packages.default;
      };
    };

    shell = mk-shell pkgs packages;
  };
in
  out
  // {
    flake = let
      inherit (nixpkgs) lib;
      supported-systems = lib.systems.doubles.linux ++ lib.systems.doubles.darwin;

      for-all-systems = function:
        lib.genAttrs supported-systems (system: function (mk-pkgs system) system);
      flake-out = {
        packages = for-all-systems (pkgs: _: mk-packages pkgs);

        devShells = for-all-systems (pkgs: system: {
          default = mk-shell pkgs flake-out.packages.${system};
        });
      };
    in
      flake-out;
  }
