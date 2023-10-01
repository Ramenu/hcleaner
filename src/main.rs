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
        const CLEAN_CACHE = 1 << 2;
        const ALWAYS_PROMPT = 1 << 3;
    }
}

macro_rules! warn {
    ($($arg:tt)*) => {{
        format!("{BOLD}{YELLOW}warning{RESET}{BOLD}:{RESET} {}", format!($($arg)*))
    }};
}

fn main() 
{
    let flags = parse_args();
    let binding = std::env::var("HOME").unwrap();
    let home = binding.as_str();
    let xdg_config = std::env::var("XDG_CONFIG_HOME").unwrap_or(format!("{}/.config", home));
    let xdg_cache = std::env::var("XDG_CACHE_HOME").unwrap_or(format!("{}/.cache", home));
    let xdg_data = std::env::var("XDG_DATA_HOME").unwrap_or(format!("{}/.local/share", home));

    let mut step = 1;
    let total_steps = || -> u64 {
        let mut total_steps = 2;
        if flags&ArgFlags::CLEAN_CACHE == ArgFlags::CLEAN_CACHE {
            total_steps += 1;
        }
        total_steps
    }();

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
        (format!("{xdg_cache}/BraveSoftware/Brave-Browser"), BRAVE_BROWSER_PKG),
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
        (format!("{home}/.idapro"), IDA_PRO_PKG),
        (format!("{xdg_config}/balena-etcher"), BALENA_ETCHER_PKG),
        (format!("{home}/.waterfox"), WATERFOX_PKG),
        (format!("{xdg_cache}/waterfox"), WATERFOX_PKG),
        (format!("{home}/.basilisk-dev/basilisk"), BASILISK_PKG),
        (format!("{xdg_cache}/basilisk-dev/basilisk"), BASILISK_PKG),
        (format!("{xdg_cache}/BraveSoftware/Brave-Browser-Nightly"), BRAVE_BROWSER_NIGHTLY_PKG),
        (format!("{xdg_config}/BraveSoftware/Brave-Browser-Nightly"), BRAVE_BROWSER_NIGHTLY_PKG),
        (format!("{xdg_config}/Session"), SESSION_PKG),
        (format!("{home}/.local/opt/tor-browser"), TOR_BROWSER_PKG),
        (format!("{xdg_config}/microsoft-edge"), MICROSOFT_EDGE_PKG),
        (format!("{xdg_cache}/microsoft-edge"), MICROSOFT_EDGE_PKG),
        (format!("{xdg_cache}/Microsoft/Edge"), MICROSOFT_EDGE_PKG),
        (format!("{home}/.netbeans"), NETBEANS_PKG),
        (format!("{xdg_cache}/netbeans"), NETBEANS_PKG),
        (format!("{home}/.sqldeveloper"), ORACLE_SQLDEVELOPER_PKG),
        (format!("{xdg_config}/JetBrains/IdeaIC2023.1"), INTELLIJ_IDEA_PKG),
        (format!("{xdg_cache}/JetBrains/IdeaIC2023.1"), INTELLIJ_IDEA_PKG),
        (format!("{home}/.gradle"), GRADLE_PKG),
        (format!("{home}/packettracer"), CISCO_PACKET_TRACER_PKG),
        (format!("{xdg_cache}/Cisco Packet Tracer"), CISCO_PACKET_TRACER_PKG),
        (format!("{xdg_data}/Cisco Packet Tracer"), CISCO_PACKET_TRACER_PKG),
        (format!("{xdg_config}/epiphany"), EPIPHANY_BROWSER_PKG),
        (format!("{xdg_cache}/epiphany"), EPIPHANY_BROWSER_PKG),
        (format!("{xdg_data}/epiphany"), EPIPHANY_BROWSER_PKG),
    ]);
    println!("{BOLD}[{step}/{total_steps}]{RESET} Checking total number of files in home directory...");
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
    step += 1;
    let bar_msg = format!("{BOLD}[{step}/{total_steps}]{RESET} Scanning files...", BOLD=BOLD, RESET=RESET);
    bar.println(bar_msg);

    let always_prompt = flags&ArgFlags::ALWAYS_PROMPT == ArgFlags::ALWAYS_PROMPT;
    let noconfirm = flags&ArgFlags::NOCONFIRM == ArgFlags::NOCONFIRM && !always_prompt;

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
                    confirm_before_exec(|| std::fs::remove_dir(path).unwrap(),
                                        always_prompt,
                                        &warn!("remove empty directory '{}'?", path.to_str().unwrap()));
                    continue;
                }
            }
        }

        bar.inc(1);
        let path_str = path.to_str().unwrap();
        let pkg = map.get(path_str);

        if let Some(pkg) = pkg {
            if let Some(false) = pkg_exists(pkg) {
                if is_dir {
                    confirm_before_exec(|| std::fs::remove_dir_all(path).unwrap(),
                                        !noconfirm,
                                        &warn!("remove all contents from '{}'?", path_str));
                } else {
                    confirm_before_exec(|| std::fs::remove_file(path).unwrap(),
                                        !noconfirm,
                                        &warn!("remove '{}'?", path_str));
                }
            }
        }
    }
    bar.finish_and_clear();

    if flags&ArgFlags::CLEAN_CACHE == ArgFlags::CLEAN_CACHE {
        step += 1;
        println!("{BOLD}[{step}/{total_steps}]{RESET} Cleaning cache...", BOLD=BOLD, RESET=RESET, step=step, total_steps=total_steps);
        clean_cache(&xdg_cache, always_prompt);
    }
}

fn clean_cache(cache_dir : &String, always_prompt : bool)
{
    let it = WalkDir::new(cache_dir).max_depth(1)
                                                                         .into_iter()
                                                                         .filter_map(|e| e.ok());
    let yay_cache = format!("{}/yay", cache_dir);
    let urlwatch_cache = format!("{}/urlwatch", cache_dir);

    for entry in it {
        let entry_path = entry.path().to_str().unwrap();

        match entry_path {
            entry_path if entry_path == cache_dir => continue,
            entry_path if entry_path == urlwatch_cache => continue,
            // yay's cache stores 'vcs.json' which can be problematic when removed, so we have to only
            // delete the directories
            entry_path if entry_path == yay_cache => {
                let it = WalkDir::new(entry.path()).max_depth(2)
                                                                                                    .into_iter()
                                                                                                    .filter_map(|e| e.ok());
                for subentry in it {
                    let subentry_path = subentry.path();
                    let subentry_str = subentry_path.to_str().unwrap();
                    if subentry_str == yay_cache || 
                       subentry_str.ends_with(".git") ||
                       subentry_path.parent().unwrap().to_str().unwrap() == yay_cache {
                        continue;
                    }
                    if !subentry_str.ends_with("PKGBUILD") {
                        confirm_before_exec(|| {
                            if subentry_path.is_dir() {
                                std::fs::remove_dir_all(subentry_path).unwrap();
                            } else {
                                std::fs::remove_file(subentry_path).unwrap();
                            }
                        },
                                            always_prompt,
                                            &warn!("remove '{}'?", subentry_str));
                    }
                }
                continue;
            }
            _ => {}
        };

        if entry.path().is_dir() {
            confirm_before_exec(|| std::fs::remove_dir_all(entry.path()).unwrap(), 
                                always_prompt, 
                            &warn!("remove all contents from '{}'?", entry.path().to_str().unwrap()));
        } else {
            confirm_before_exec(|| std::fs::remove_file(entry.path()).unwrap(), 
                                always_prompt, 
                                &warn!("remove '{}'?", entry.path().to_str().unwrap()));
        }
    }
}

fn confirm_before_exec<T>(callback : T, confirm : bool, msg : &str)
    where T : Fn()
{
    if confirm {
        print!("{msg} [y/N] ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() != "y" {
            return;
        }
    }
    callback();
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
                            "--clean-cache" => flags |= ArgFlags::CLEAN_CACHE,
                            "--always-prompt" => flags |= ArgFlags::ALWAYS_PROMPT,
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
