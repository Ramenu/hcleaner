use std::fs::File;
use std::io::{self, BufRead};
use lazy_static::lazy_static;

use crate::log;

pub struct DistroInfo 
{
    pub name : &'static str,
    pub base_flag : u32
}

lazy_static! {
    pub static ref DISTRIBUTION : DistroInfo = check_linux_distribution();
}

pub const ARCH_BASE_FLAG : u32 = 1;
pub const DEBIAN_BASE_FLAG : u32 = 2;

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
                        ARCH_STR => return DistroInfo{name: ARCH_STR, base_flag: ARCH_BASE_FLAG},
                        DEBIAN_STR => return DistroInfo{name: DEBIAN_STR, base_flag: DEBIAN_BASE_FLAG},
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