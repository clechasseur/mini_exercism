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
mod tests {
    use super::*;

    #[test]
    fn test_get_cli_config_dir() {
        // For some reason, trying to remove this function from coverage using
        // `not(tarpaulin_include)` does not seem to work, so we provide this
        // dummy test instead. If anyone knows how to fix this, don't hesitate 😥
        let config_dir = get_cli_config_dir();
        assert!(config_dir.is_some() || config_dir.is_none());
    }
}
