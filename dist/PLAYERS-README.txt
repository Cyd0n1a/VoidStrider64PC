VOIDSTRIDER64
=============

How to play
-----------
Windows:  double-click VoidStrider64.exe
Linux:    run ./VoidStrider64 from this folder
          (first time you may need: chmod +x VoidStrider64)

The game starts automatically. Keep the .z64 file in the same folder
as the executable.

Controls
--------
A gamepad (Xbox/PlayStation/etc.) is picked up automatically.
Keyboard defaults are also available out of the box.

To remap controls, open a terminal in this folder and run:
  VoidStrider64 --configure-input-profile myprofile
  VoidStrider64 --bind-input-profile myprofile --port 1

Useful options
--------------
  --fullscreen        start in fullscreen
  --help              list all options

Requirements
------------
- Windows 10 or newer, or a Linux desktop (X11 or Wayland)
- A GPU with Vulkan support (any reasonably modern GPU/driver)

Saves
-----
Your save data is stored per-user, not in this folder:
  Windows:  %APPDATA%\gopher64\saves
  Linux:    ~/.local/share/gopher64/saves

Credits
-------
This game runs on a modified build of the open-source Gopher64
N64 emulator (GPL-3.0). Emulator source code for this build:
  https://github.com/Cyd0n1a/VoidStrider64PC
Upstream project: https://github.com/gopher64/gopher64
See EMULATOR-LICENSE.txt for the emulator's license.
