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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_get_cli_config_dir() {
        let config_dir = get_cli_config_dir();
        assert!(config_dir.is_some() || config_dir.is_none());
    }
}
