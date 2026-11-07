{pkgs, ...}: {
  packages = with pkgs; [
    clang
    mold
    bacon
    rust-analyzer
    cargo-nextest
    cmake
    wayland
    libx11
    libxrandr
    libxinerama
    libxcursor
    libxi
    libGL
    glfw

    just
    watchexec
  ];
  env = {
    LD_LIBRARY_PATH = with pkgs;
      lib.makeLibraryPath [libGL];
    LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  };
}
