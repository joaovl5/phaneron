{
  pkgs,
  project-package,
  vm-memory ? 2048,
  vm-cores ? 4,
  vm-test-user ? "test-steve",
}: let
  host-nixpkgs-path = pkgs.path;

  # niri doesn't like software rendering and thus needs a patch
  niri-patched = pkgs.niri.overrideAttrs (old: {
    postPatch =
      (old.postPatch or "")
      # sh
      + ''
        substituteInPlace src/backend/tty.rs \
          --replace-fail '!egl_device.is_software(),' 'true,'
      '';
  });
in
  pkgs.testers.runNixOSTest {
    name = "project-test";

    nodes.machine = {pkgs, ...}: {
      imports = ["${host-nixpkgs-path}/nixos/tests/common/user-account.nix"];
      hardware.graphics.enable = true;
      services.seatd.enable = true;
      users.users.${vm-test-user}.extraGroups = ["seat" "video"];
      services.getty.autologinUser = vm-test-user;
      environment = {
        systemPackages = [project-package];
        # kdl
        etc."niri/config.kdl".text = ''
          hotkey-overlay { skip-at-startup }
          spawn-sh-at-startup "exec ${pkgs.getExe project-package}"
        '';
      };
      programs = {
        niri.enable = true;
        niri.package = niri-patched;
        # sh
        bash.loginShellInit = ''
          if [ "$(tty)" = "/dev/tty1" ]; then
            exec niri --session
          fi
        '';
      };
      virtualisation = {
        memorySize = vm-memory;
        cores = vm-cores;
      };
    };

    # python
    testScript = ''
      start_all()

      with subtest("executable starts"):
        machine.wait_for_unit("multi-user.target")
        machine.succeed("niri validate --config /etc/niri/config.kdl")
        machine.wait_until_succeeds(
          "test -e /run/user/$(id -u ${vm-test-user})",
          timeout=120,
        )
    '';
  }
