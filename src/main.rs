#![deny(warnings)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use clap::Parser;

/// ROM file extensions we will auto-load, in order of preference.
const ROM_EXTENSIONS: [&str; 5] = ["z64", "n64", "v64", "zip", "7z"];

/// Standalone-launcher mode: look for a ROM sitting next to the executable
/// (falling back to the current working directory) so this binary can be
/// shipped in a folder together with a single game and "just run" it when
/// double-clicked, with no arguments required.
fn find_bundled_rom() -> Option<String> {
    let mut search_dirs: Vec<std::path::PathBuf> = Vec::new();
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        search_dirs.push(dir.to_path_buf());
    }
    if let Ok(cwd) = std::env::current_dir() {
        search_dirs.push(cwd);
    }

    for dir in search_dirs {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        let mut candidates: Vec<std::path::PathBuf> = entries
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| {
                path.is_file()
                    && path.extension().is_some_and(|ext| {
                        ROM_EXTENSIONS
                            .iter()
                            .any(|known| ext.eq_ignore_ascii_case(known))
                    })
            })
            .collect();
        // Sort so the pick is deterministic if several ROMs are present,
        // and prefer .z64 over compressed containers.
        candidates.sort_by_key(|path| {
            let ext_rank = path
                .extension()
                .and_then(|ext| {
                    ROM_EXTENSIONS
                        .iter()
                        .position(|known| ext.eq_ignore_ascii_case(known))
                })
                .unwrap_or(usize::MAX);
            (ext_rank, path.clone())
        });
        if let Some(rom) = candidates.first() {
            return Some(rom.to_string_lossy().into_owned());
        }
    }
    None
}

fn main() -> std::io::Result<()> {
    let (close_tx, handle) = gopher64::create_runtime();
    let _guard = handle.enter();

    let mut args = gopher64::Args::parse();

    // If no game was given on the command line and the user isn't running a
    // configuration command, launch the ROM that ships alongside this binary.
    // All original CLI functionality (input profile setup, fullscreen, etc.)
    // is preserved.
    if args.game.is_none()
        && args.configure_input_profile.is_none()
        && args.bind_input_profile.is_none()
        && args.assign_controller.is_none()
        && !args.list_controllers
        && !args.clear_input_bindings
        && let Some(rom) = find_bundled_rom()
    {
        args.game = Some(rom);
    }

    let result = gopher64::run(args, std::env::args().count());
    close_tx.send(()).unwrap();
    result
}
