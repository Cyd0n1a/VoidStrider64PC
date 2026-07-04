# VoidStrider64 standalone launcher — setup guide

This folder contains everything needed to turn a fork of
[Gopher64](https://github.com/gopher64/gopher64) into a standalone,
double-clickable launcher for your game on Linux (X11/Wayland) and
Windows 10+.

## How it works

One small change to `src/main.rs`: when the emulator is started with no
ROM argument, it now looks for a `.z64` / `.n64` / `.v64` (or `.zip` /
`.7z`) file next to the executable and boots it directly — the emulator
GUI never appears. All of Gopher64's command-line options (fullscreen,
input remapping, save states) still work, so players keep an escape
hatch for configuring controls.

The GitHub Actions workflow (`.github/workflows/build-game.yml`) builds
release binaries for Linux x86_64 and Windows x86_64 on GitHub's
servers, packages each with your ROM and a player README, and attaches
the archives to a GitHub Release whenever you push a version tag.

## One-time setup

1. Fork https://github.com/gopher64/gopher64 on GitHub (keep the fork
   public — see the license note below).

2. Clone your fork with submodules:

       git clone --recursive https://github.com/Cyd0n1a/VoidStrider64PC/
       cd VoidStrider64PC

3. Copy the files from this folder into the repo:

       src/main.rs                          -> replaces src/main.rs
                                               (or: git apply main.rs.patch)
       .github/workflows/build-game.yml     -> new file
       dist/PLAYERS-README.txt              -> new file
       game/voidstrider64.z64               -> your ROM (rename freely,
                                               keep the .z64 extension)

   The ROM is ~40 MB, comfortably under GitHub's 100 MB per-file limit,
   so committing it directly is fine.

4. Edit two placeholders:
   - `dist/PLAYERS-README.txt`: put your real fork URL in the Credits
     section.
   - `build-game.yml`: change `GAME_NAME` if you want a different
     executable/archive name.

5. Commit and push:

       git add -A
       git commit -m "Turn Gopher64 into VoidStrider64 standalone launcher"
       git push

6. In your fork's Settings -> Actions, make sure Actions are enabled.

## Shipping a release

    git tag v0.6a
    git push origin v0.6a

Wait ~15–30 minutes and a GitHub Release appears containing:

    VoidStrider64-linux-x86_64.tar.gz
    VoidStrider64-windows-x86_64.zip

Each archive is a single folder with the executable, your ROM, a player
README, and the emulator license. Players unzip and double-click —
nothing to install. You can also trigger a build without tagging via
the Actions tab ("Run workflow"), which produces downloadable artifacts
instead of a release.

## Updating your game

Replace the file in `game/`, commit, tag a new version, push the tag.

## Pulling in upstream emulator improvements

    git remote add upstream https://github.com/gopher64/gopher64.git
    git fetch upstream
    git merge upstream/main

The launcher change is small and self-contained in `src/main.rs`, so
merges should rarely conflict.

## Platform notes

- **Linux/Wayland**: Gopher64 uses SDL3, which supports Wayland
  natively (with X11 fallback), so a single Linux binary covers both.
  Players need a Vulkan-capable GPU/driver — that's the emulator's
  rendering backend (paraLLEl-RDP).
- **Windows**: the release build already hides the console window.
  Unsigned executables will trigger a SmartScreen "unknown publisher"
  prompt on first run; that's normal for indie distribution (players
  click "More info" -> "Run anyway"). Code signing removes it but isn't
  required.
- **ARM builds**: upstream's CI also builds linux-aarch64 and
  windows-aarch64; if you want those, duplicate the jobs in
  `build-game.yml` using the matrix entries from upstream's
  `.github/workflows/build.yml`.

## License note (GPL-3.0)

Gopher64 is GPL-3.0. That means your **fork of the emulator** must stay
GPL-3.0 and its source must be available to anyone you distribute
binaries to — a public GitHub fork satisfies this, and linking to it in
the player README (already done) is good practice. Your **ROM is not
affected**: it's separate data loaded at runtime, not linked code, so
your game keeps whatever license you choose.
