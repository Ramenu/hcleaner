use std::{collections::HashMap, io::Write};

use indicatif::{ProgressBar, ProgressStyle};
use os::DISTRIBUTION;
use package::*;
use walkdir::WalkDir;
use bitflags::bitflags;

use crate::constants::*;

mod os;
mod log;
mod package;
mod constants;

bitflags! {
    #[derive(PartialEq, Clone, Copy)]
    struct ArgFlags : u32 {
        const NONE = 0;
        const NOCONFIRM = 1 << 0;
        const SHOW_VERSION = 1 << 1;
    }
}

fn main() 
{
    let flags = parse_args();
    let binding = std::env::var("HOME").unwrap();
    let home = binding.as_str();
    let xdg_config = std::env::var("XDG_CONFIG_HOME").unwrap_or(format!("{}/.config", home));
    let xdg_cache = std::env::var("XDG_CACHE_HOME").unwrap_or(format!("{}/.cache", home));
    let xdg_data = std::env::var("XDG_DATA_HOME").unwrap_or(format!("{}/.local/share", home));

    let map : HashMap<String, Package> = HashMap::from([
        (format!("{home}/.dillo"), DILLO_PKG),
        (format!("{xdg_config}/supertuxkart"), SUPERTUXKART_PKG),
        (format!("{xdg_data}/supertuxkart"), SUPERTUXKART_PKG),
        (format!("{xdg_config}/syncthing"), SYNCTHING_PKG),
        (format!("{xdg_config}/midori"), MIDORI_PKG),
        (format!("{xdg_config}/lapce"), LAPCE_PKG),
        (format!("{xdg_config}/weechat"), WEECHAT_PKG),
        (format!("{xdg_data}/weechat"), WEECHAT_PKG),
        (format!("{xdg_data}/midori"), MIDORI_PKG),
        (format!("{xdg_config}/neofetch"), NEOFETCH_PKG),
        (format!("{xdg_config}/BraveSoftware/Brave-Browser"), BRAVE_BROWSER_PKG),
        (format!("{xdg_config}/libreoffice"), LIBREOFFICE_PKG),
        (format!("{xdg_config}/VirtualBox"), VIRTUALBOX_PKG),
        (format!("{home}/VirtualBox VMs"), VIRTUALBOX_PKG),
        (format!("{xdg_config}/GIMP"), GIMP_PKG),
        (format!("{xdg_config}/keepassxc"), KEEPASSXC_PKG),
        (format!("{xdg_cache}/keepassxc"), KEEPASSXC_PKG),
        (format!("{xdg_config}/Code"), VSCODE_PKG),
        (format!("{xdg_config}/Code - OSS"), VSCODE_PKG),
        (format!("{home}/.vscode"), VSCODE_PKG),
        (format!("{home}/.mozilla/firefox"), FIREFOX_PKG),
        (format!("{xdg_cache}/mozilla/firefox"), FIREFOX_PKG),
        (format!("{xdg_config}/vivaldi"), VIVALDI_PKG),
        (format!("{xdg_config}/yay"), YAY_PKG),
        (format!("{xdg_cache}/yay"), YAY_PKG),
        (format!("{xdg_config}/unity3d"), UNITY_PKG),
        (format!("{xdg_data}/lapce-stable"), LAPCE_PKG),
        (format!("{xdg_data}/flatpak"), FLATPAK_PKG),
        (format!("{xdg_cache}/flatpak"), FLATPAK_PKG),
        (format!("{home}/.var/app"), FLATPAK_PKG),
        (format!("{home}/.mullvad/mullvadbrowser"), MULLVAD_BROWSER_PKG),
        (format!("{xdg_cache}/mullvad/mullvadbrowser"), MULLVAD_BROWSER_PKG),
        (format!("{xdg_data}/dolphin"), DOLPHIN_PKG),
        (format!("{xdg_config}/dolphinrc"), DOLPHIN_PKG),
        (format!("{xdg_data}/dolphin-emu"), DOLPHIN_EMU_PKG),
        (format!("{xdg_cache}/dolphin-emu"), DOLPHIN_EMU_PKG),
        (format!("{xdg_config}/dolphin-emu"), DOLPHIN_EMU_PKG),
        (format!("{xdg_config}/desmume"), DESMUME_PKG),
        (format!("{xdg_config}/discord"), DISCORD_PKG),
        (format!("{home}/.thunderbird"), THUNDERBIRD_PKG),
        (format!("{home}/.rustup"), RUSTUP_PKG),
        (format!("{home}/.cargo"), RUSTUP_PKG),
        (format!("{home}/.parallel"), PARALLEL_PKG),
        (format!("{home}/.mplayer"), MPLAYER_PKG),
        (format!("{home}/.cmake"), CMAKE_PKG),
        (format!("{home}/go"), GO_PKG),
        (format!("{xdg_cache}/go-build"), GO_PKG),
        (format!("{xdg_cache}/paru"), PARU_PKG),
        (format!("{xdg_config}/paru"), PARU_PKG),
        (format!("{xdg_cache}/chromium"), CHROMIUM_PKG),
        (format!("{xdg_config}/chromium"), CHROMIUM_PKG),
        (format!("{xdg_data}/baloo"), BALOO_PKG),
        (format!("{xdg_config}/Signal"), SIGNAL_PKG),
        (format!("{xdg_config}/google-chrome"), GOOGLE_CHROME_PKG),
        (format!("{xdg_cache}/google-chrome"), GOOGLE_CHROME_PKG),
        (format!("{home}/.librewolf"), LIBREWOLF_PKG),
        (format!("{xdg_cache}/librewolf"), LIBREWOLF_PKG),
    ]);
    println!("{BOLD}[1/2]{RESET} Checking total number of files in home directory...");
    let total_files = WalkDir::new(home).into_iter()
                                                    .filter_map(|e| e.ok())
                                                    .count();

    let it = WalkDir::new(home).into_iter()
                                                                    .filter_map(|e| e.ok());
    let bar = ProgressBar::new(total_files as u64);
    let (primary_accent_color, secondary_accent_color) = get_accent_colors();
    let bar_template = format!("[{{elapsed_precise}}] [{{bar:40.{primary_accent_color}/{secondary_accent_color}}}] {{pos:>7}}/{{len:7}} {{msg}}"); 
    bar.set_style(ProgressStyle::default_bar()
                                .template(&bar_template)
                                .unwrap()
                                .progress_chars("##-"));
    let bar_msg = format!("{BOLD}[2/2]{RESET} Scanning files...", BOLD=BOLD, RESET=RESET);
    bar.println(bar_msg);

    for entry in it {
        let path = entry.path();
        let is_dir = path.is_dir();

        // Remove empty directories without prompt as they are
        // just extra clutter
        if is_dir {
            let read_dir = path.read_dir();
            if let Ok(mut read_dir) = read_dir {
                let is_empty = read_dir.next().is_none();
                if is_empty {
                    std::fs::remove_dir(path).unwrap();
                    continue;
                }
            }
        }

        bar.inc(1);
        let path_str = path.to_str().unwrap();
        let pkg = map.get(path_str);

        if let Some(pkg) = pkg {
            let exists = pkg_exists(pkg);

            if exists.is_some() && !exists.unwrap() {
                if flags&ArgFlags::NOCONFIRM == ArgFlags::NONE {
                    print!("remove '{}'? [y/N] ", path_str);
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();

                    if input.trim() != "y" {
                        continue;
                    }
                }

                if is_dir {
                    println!("removing all contents from '{}'", path_str);
                    std::fs::remove_dir_all(path).unwrap();
                } else {
                    println!("removing '{}'", path_str);
                    std::fs::remove_file(path).unwrap();
                }
            }
        }
    }
    bar.finish();
}

/// Parses the command line arguments passed to the program and
/// returns a flag. Note that we can inline this since this is
/// only called once in the beginning of the program.
#[inline]
fn parse_args() -> ArgFlags
{
    let args = std::env::args().collect::<Vec<String>>();
    let mut flags = ArgFlags::NONE;

    for (i, arg) in args.iter().enumerate() {
        let mut starts_flag = false;
        for c in arg.chars() {
            if c == '-' && !starts_flag {
                starts_flag = true;
                continue;
            }
            else if !starts_flag && i != 0 {
                log::terminate("unknown option specified");
            }
            if starts_flag {
                match c {
                    // show version 
                    'v' => flags |= ArgFlags::SHOW_VERSION,
                    // more verbose commands
                    '-' => {
                        match arg.as_str() {
                            "--version" => flags |= ArgFlags::SHOW_VERSION,
                            "--noconfirm" => flags |= ArgFlags::NOCONFIRM,
                            "--help" => {
                                println!("{}", HELP_MESSAGE);
                                std::process::exit(0);
                            },
                            _ => {
                                log::terminate("unknown option specified");
                            }
                        }
                        break;
                    },
                    // unknown flag
                    _ => {
                        log::terminate("unknown option specified");
                    }
                };
            }
        }
    }

    if flags&ArgFlags::SHOW_VERSION == ArgFlags::SHOW_VERSION {
        show_version();
        std::process::exit(0);
    }

    flags
}

fn get_accent_colors() -> (&'static str, &'static str)
{
    match DISTRIBUTION.name {
        "Arch Linux" => ("cyan", "blue"),
        "Debian GNU/Linux" => ("red", "blue"),
        _ => ("white", "white"),
    }
}

#[inline]
fn show_version()
{
    println!("hcleaner v{}\n\nbuilt using {}", env!("CARGO_PKG_VERSION"), RUSTC_VERSION);
}
