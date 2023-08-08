
pub const RUSTC_VERSION : &str = env!("rustc_version");
pub const HELP_MESSAGE : &str = "
Usage: hcleaner [OPTION]...\n\
A program for removing configuration files and directories of uninstalled packages\n\
\n\
Options:\n\
    -v, --version    show version information and exit\n\
    --noconfirm      do not prompt for confirmation before deleting directories and\n\
                     files\n\
    --help           show this help message and exit\n\
\n\
Report bugs and security issues on https://www.github.com/Ramenu/hcleaner";

pub const RESET : &str = "\x1b[0m";
pub const RED : &str = "\x1b[31m";
pub const GREEN : &str = "\x1b[32m";
pub const YELLOW : &str = "\x1b[33m";
pub const BLUE : &str = "\x1b[34m";
pub const MAGENTA : &str = "\x1b[35m";
pub const CYAN : &str = "\x1b[36m";
pub const WHITE : &str = "\x1b[37m";
pub const BOLD : &str = "\x1b[1m";