use std::fs::File;
use std::io::{self, BufRead};
use lazy_static::lazy_static;

use crate::log;

pub enum DistroBase
{
    Arch,
    Debian
}

pub struct DistroInfo 
{
    pub name : &'static str,
    pub base : DistroBase
}

lazy_static! {
    pub static ref DISTRIBUTION : DistroInfo = check_linux_distribution();
}

pub const ARCH_STR : &str = "Arch Linux";
pub const DEBIAN_STR : &str = "Debian GNU/Linux";

fn check_linux_distribution() -> DistroInfo
{
    if let Ok(file) = File::open("/etc/os-release") {
        let reader = io::BufReader::new(file);

        for line in reader.lines().flatten() {
            if line.starts_with("NAME=") {
                // Extract the distribution name between double quotes
                if let Some(name) = line.split_once('=').map(|x| x.1) {
                    let name = name.trim_matches('"');
                    match name {
                        ARCH_STR => return DistroInfo{name: ARCH_STR, base: DistroBase::Arch},
                        DEBIAN_STR => return DistroInfo{name: DEBIAN_STR, base: DistroBase::Debian},
                        _ => log::terminate("unsupported Linux distribution")
                    };
                    
                }
            }
        }
    } else {
        log::terminate("cannot open '/etc/os-release'");
    }

    log::terminate("cannot find the Linux distribution name");
}