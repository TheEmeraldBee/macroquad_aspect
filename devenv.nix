{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:
let
  shared = with pkgs; [
    alsa-lib
    udev
    wayland
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libxcb
    libGL
    vulkan-loader
    vulkan-headers
    libxkbcommon
  ];
in
{
  packages = shared;
  env.LD_LIBRARY_PATH = lib.makeLibraryPath shared;

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}
