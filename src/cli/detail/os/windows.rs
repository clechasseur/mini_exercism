use std::env;
use std::path::PathBuf;

pub fn get_cli_config_dir() -> Option<PathBuf> {
    // Based on:
    // https://github.com/exercism/cli/blob/9e1285b62502f3f5a4a896a44e540ee1bee5c1bf/config/config.go#L57-L60

    let mut path: PathBuf = env::var_os("APPDATA")?.into();
    path.push("exercism");
    Some(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_cli_config_dir {
        use std::path::MAIN_SEPARATOR;

        use serial_test::serial;

        use super::*;

        #[test]
        #[serial]
        fn test_valid() {
            let app_data = r"C:\Users\some_user\AppData\Roaming";
            env::set_var("APPDATA", app_data);
            let config_dir = get_cli_config_dir();

            assert_eq!(
                config_dir,
                Some(format!("{}{}{}", app_data, MAIN_SEPARATOR, "exercism").into())
            );
        }

        #[test]
        #[serial]
        fn test_invalid() {
            env::remove_var("APPDATA");
            let config_dir = get_cli_config_dir();

            assert_eq!(config_dir, None);
        }
    }
}
