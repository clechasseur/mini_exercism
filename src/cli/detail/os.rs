#[cfg(any(not(windows), test))]
mod nix;
#[cfg(any(windows, test))]
mod windows;

use std::path::PathBuf;

#[cfg(not(windows))]
pub fn get_cli_config_dir() -> Option<PathBuf> {
    nix::get_cli_config_dir()
}

#[cfg(windows)]
pub fn get_cli_config_dir() -> Option<PathBuf> {
    windows::get_cli_config_dir()
}
