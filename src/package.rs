use std::process::Command;

use crate::os::{DISTRIBUTION, ARCH_BASE_FLAG};

pub const DILLO_PKG : Package = Package{arch: "dillo", debian: "dillo"};
pub const SUPERTUXKART_PKG : Package = Package{arch: "supertuxkart", debian: "supertuxkart"};
pub const SYNCTHING_PKG : Package = Package{arch: "syncthing", debian: "syncthing"};
pub const MIDORI_PKG : Package = Package{arch: "midori", debian: "midori"};
pub const LAPCE_PKG : Package = Package{arch: "lapce", debian: ""};
pub const WEECHAT_PKG : Package = Package{arch: "weechat", debian: "weechat"};
pub const NEOFETCH_PKG : Package = Package{arch: "neofetch", debian: "neofetch"};
pub const BRAVE_BROWSER_PKG : Package = Package{arch: "brave-bin", debian: ""};
pub const LIBREOFFICE_PKG : Package = Package{arch: "libreoffice-fresh", debian: "libreoffice"};
pub const VIRTUALBOX_PKG : Package = Package{arch: "virtualbox", debian: "virtualbox"};
pub const GIMP_PKG : Package = Package{arch: "gimp", debian: "gimp"};
pub const KEEPASSXC_PKG : Package = Package{arch: "keepassxc", debian: "keepassxc"};
pub const VSCODE_PKG : Package = Package{arch: "visual-studio-code-bin", debian: ""};
pub const FIREFOX_PKG : Package = Package{arch: "firefox", debian: "firefox"};
pub const VIVALDI_PKG : Package = Package{arch: "vivaldi", debian: ""};
pub const YAY_PKG : Package = Package{arch: "yay|yay-git|yay-bin", debian: ""};
pub const UNITY_PKG : Package = Package{arch: "unityhub", debian: ""};
pub const FLATPAK_PKG : Package = Package{arch: "flatpak", debian: "flatpak"};
pub const MULLVAD_BROWSER_PKG : Package = Package{arch: "mullvad-browser-bin", debian: ""};
pub const DOLPHIN_PKG : Package = Package{arch: "dolphin", debian: "dolphin"};
pub const DOLPHIN_EMU_PKG : Package = Package{arch: "dolphin-emu", debian: "dolphin-emu"};
pub const DESMUME_PKG : Package = Package{arch: "desmume", debian: "desmume"};
pub const DISCORD_PKG : Package = Package{arch: "discord", debian: ""};
pub const THUNDERBIRD_PKG : Package = Package{arch: "thunderbird", debian: "thunderbird"};
pub const RUSTUP_PKG : Package = Package{arch: "rustup", debian: ""};
pub const PARALLEL_PKG : Package = Package{arch: "parallel", debian: "parallel"};
pub const MPLAYER_PKG : Package = Package{arch: "mplayer", debian: "mplayer"};
pub const CMAKE_PKG : Package = Package{arch: "cmake", debian: "cmake"};
pub const GO_PKG : Package = Package{arch: "go", debian: "golang"};
pub const PARU_PKG : Package = Package{arch: "paru", debian: ""};
pub const CHROMIUM_PKG : Package = Package{arch: "chromium|ungoogled-chromium|ungoogled-chromium-bin", debian: "chromium"};
pub const BALOO_PKG : Package = Package{arch: "baloo", debian: "baloo-kf5"};
pub const SIGNAL_PKG : Package = Package{arch: "signal-desktop", debian: ""};

pub struct Package
{
    pub arch : &'static str,
    pub debian : &'static str,
}

/// Should be `Some(...)` if the package is available
/// on the distribution. If the unwrapped value
/// is false, then the package is not installed.
pub fn pkg_exists(pkg : &Package) -> Option<bool>
{
    match DISTRIBUTION.base_flag {
        ARCH_BASE_FLAG => pkg_exists_arch(pkg.arch),
        _ => unreachable!()
    }
}

fn pkg_exists_arch(pkgname : &str) -> Option<bool>
{
    if pkgname.is_empty() {
        return None
    }

    let all_pkgs = pkgname.split('|');

    for pkgname in all_pkgs {
        let output = Command::new("pacman")
                                     .arg("-Q")
                                     .arg(pkgname)
                                     .output()
                                     .expect("Failed to execute pacman command");

        if output.status.success() {
            return Some(true)
        }
    }

    Some(false)
}
