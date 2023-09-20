
pub const RUSTC_VERSION : &str = env!("rustc_version");
pub const HELP_MESSAGE : &str = "
Usage: hcleaner [OPTION]...\n\
A program for removing configuration files and directories of uninstalled packages\n\
in the user's home directory.\n\
\n\
Options:\n\
    -v, --version    show version information and exit\n\
    --noconfirm      do not prompt for confirmation before deleting directories and
                 files\n\
    --clean-cache    clean the ${XDG_CACHE_DIR} directory\n\
    --always-prompt  always prompt for confirmation before deleting directories and
                 files (this overrides --noconfirm)\n\
    --help           show this help message and exit\n\
\n\
Report bugs and security issues on https://www.github.com/Ramenu/hcleaner";

#[allow(unused)] pub const RESET : &str = "\x1b[0m";
#[allow(unused)] pub const RED : &str = "\x1b[31m";
#[allow(unused)] pub const GREEN : &str = "\x1b[32m";
#[allow(unused)] pub const YELLOW : &str = "\x1b[33m";
#[allow(unused)] pub const BLUE : &str = "\x1b[34m";
#[allow(unused)] pub const MAGENTA : &str = "\x1b[35m";
#[allow(unused)] pub const CYAN : &str = "\x1b[36m";
#[allow(unused)] pub const WHITE : &str = "\x1b[37m";
#[allow(unused)] pub const BOLD : &str = "\x1b[1m";