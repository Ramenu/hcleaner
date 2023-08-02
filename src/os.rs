use std::fs::File;
use std::io::{self, BufRead};
use lazy_static::lazy_static;

use crate::log;


lazy_static! {
    pub static ref DISTRIBUTION : u32 = check_linux_distribution();
}

pub const ARCH : u32 = 1;
pub const DEBIAN : u32 = 2;

fn check_linux_distribution() -> u32
{
    if let Ok(file) = File::open("/etc/os-release") {
        let reader = io::BufReader::new(file);

        for line in reader.lines().flatten() {
            if line.starts_with("NAME=") {
                // Extract the distribution name between double quotes
                if let Some(name) = line.split_once('=').map(|x| x.1) {
                    let name = name.trim_matches('"');
                    match name {
                        "Arch Linux" => return ARCH,
                        "Debian GNU/Linux" => return DEBIAN,
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