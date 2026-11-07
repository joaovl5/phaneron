{pkgs, ...}: let
  src-path = ../../.;
  workspace-cfg =
    ../../Cargo.toml |> builtins.readFile |> builtins.fromTOML;
  inherit (workspace-cfg.workspace.metadata) project-name;
  rust-toolchain =
    ../../rust-toolchain.toml
    |> pkgs.rust-bin.fromRustupToolchainFile;
  rust-platform = pkgs.makeRustPlatform {
    cargo = rust-toolchain;
    rustc = rust-toolchain;
  };
  # any runtime libs needed can be put here
  runtime-libs = with pkgs; [
    wayland
    libx11
    libxrandr
    libxinerama
    libxcursor
    libxi
    libGL
    glfw
  ];
  src =
    src-path
    |> pkgs.lib.fileset.gitTracked
    |> (x: {
      root = src-path;
      fileset = x;
    })
    |> pkgs.lib.fileset.toSource;
in {
  inherit rust-toolchain;

  default = rust-platform.buildRustPackage {
    inherit src;
    pname = project-name;
    meta.mainProgram = project-name;
    meta.description = workspace-cfg.workspace.package.description;
    version = workspace-cfg.workspace.package.version;
    cargoLock.lockFile = ../../Cargo.lock;
    useNextest = true;
    cargoTestFlags = ["--workspace"];
    nativeBuildInputs = with pkgs; [
      clang
      cmake
      mold
      makeWrapper
      pkg-config

      wayland
      libx11
      libxrandr
      libxinerama
      libxcursor
      libxi
      libGL
      glfw
    ];
    postFixup = ''
      wrapProgram $out/bin/${project-name} \
        --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath runtime-libs}
    '';

    # if cargo can't find a "main" crate this should be here:
    # cargoBuildFlags = ["-p" "the-package-name"];
  };
}
