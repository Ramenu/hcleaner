
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