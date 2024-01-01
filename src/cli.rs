//! Utilities to interact with the [Exercism CLI application](https://exercism.org/docs/using/solving-exercises/working-locally).

mod detail;

use std::{env, io};

use mockall_double::double;

#[double]
use crate::cli::detail::helpers;
use crate::cli::detail::CliConfig;
use crate::core::{Credentials, Error, Result};

/// Reads API credentials from the CLI config file and returns them.
///
/// # Errors
///
/// - [`ConfigNotFound`]: CLI config file cannot be found, maybe CLI is not installed
/// - [`ConfigReadError`]: I/O error reading the config file
/// - [`ConfigParseError`]: Config file JSON could not be parsed
/// - [`ApiTokenNotFoundInConfig`]: Config file did not contain an API token
///
/// [`ConfigNotFound`]: crate::Error::ConfigNotFound
/// [`ConfigReadError`]: crate::Error::ConfigReadError
/// [`ConfigParseError`]: crate::Error::ConfigParseError
/// [`ApiTokenNotFoundInConfig`]: crate::Error::ApiTokenNotFoundInConfig
pub fn get_cli_credentials() -> Result<Credentials> {
    let mut config_file_path = helpers::get_cli_config_dir()
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
        .or_else(|_| env::current_dir())?;
    config_file_path.push("user.json");

    match helpers::read_to_string(&config_file_path) {
        Ok(config) => {
            let config = CliConfig::from_string(config.as_str())?;
            Ok(Credentials::from_api_token(config.api_token))
        },
        Err(err) if err.kind() == io::ErrorKind::NotFound => Err(Error::ConfigNotFound),
        Err(err) => Err(Error::from(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_cli_credentials {
        use std::path::PathBuf;

        use assert_matches::assert_matches;
        use mockall::predicate::eq;
        use serial_test::serial;

        use super::*;

        #[test]
        #[serial(cli_rs_get_cli_credentials)]
        fn test_valid() {
            let expected_config_dir = "/some/config/dir".into();
            let gccd_ctx = helpers::get_cli_config_dir_context();
            gccd_ctx
                .expect()
                .return_once(move || Some(expected_config_dir));

            let expected_config_path: PathBuf = ["/some/config/dir", "user.json"].iter().collect();
            let expected_json_file = "{\"token\": \"some_token\"}".into();
            let rts_ctx = helpers::read_to_string_context();
            rts_ctx
                .expect()
                .with(eq(expected_config_path))
                .return_once(move |_| Ok(expected_json_file));

            assert_matches!(get_cli_credentials(),
                Ok(creds) if creds == Credentials::from_api_token("some_token"));
        }

        #[test]
        #[serial(cli_rs_get_cli_credentials)]
        fn test_no_config_dir() {
            let gccd_ctx = helpers::get_cli_config_dir_context();
            gccd_ctx.expect().return_once(|| None);

            let expected_config_path: PathBuf = [env::current_dir().unwrap(), "user.json".into()]
                .iter()
                .collect();
            let expected_json_file = "{\"token\": \"some_token\"}".into();
            let rts_ctx = helpers::read_to_string_context();
            rts_ctx
                .expect()
                .with(eq(expected_config_path))
                .return_once(move |_| Ok(expected_json_file));

            assert_matches!(get_cli_credentials(),
                Ok(creds) if creds == Credentials::from_api_token("some_token"));
        }

        #[test]
        #[serial(cli_rs_get_cli_credentials)]
        fn test_invalid_config() {
            let expected_config_dir = "/some/config/dir".into();
            let gccd_ctx = helpers::get_cli_config_dir_context();
            gccd_ctx
                .expect()
                .return_once(move || Some(expected_config_dir));

            let expected_config_path: PathBuf = ["/some/config/dir", "user.json"].iter().collect();
            let expected_json_file = "{invalid: json}".into();
            let rts_ctx = helpers::read_to_string_context();
            rts_ctx
                .expect()
                .with(eq(expected_config_path))
                .return_once(move |_| Ok(expected_json_file));

            assert_matches!(get_cli_credentials(),
                Err(Error::ConfigParseError(json_error)) if json_error.is_syntax());
        }

        #[test]
        #[serial(cli_rs_get_cli_credentials)]
        fn test_config_file_not_found() {
            let expected_config_dir = "/some/config/dir".into();
            let gccd_ctx = helpers::get_cli_config_dir_context();
            gccd_ctx
                .expect()
                .return_once(move || Some(expected_config_dir));

            let expected_config_path: PathBuf = ["/some/config/dir", "user.json"].iter().collect();
            let rts_ctx = helpers::read_to_string_context();
            rts_ctx
                .expect()
                .with(eq(expected_config_path))
                .return_once(|_| Err(io::Error::from(io::ErrorKind::NotFound)));

            assert_matches!(get_cli_credentials(), Err(Error::ConfigNotFound));
        }

        #[test]
        #[serial(cli_rs_get_cli_credentials)]
        fn test_config_file_inaccessible() {
            let expected_config_dir = "/some/config/dir".into();
            let gccd_ctx = helpers::get_cli_config_dir_context();
            gccd_ctx
                .expect()
                .return_once(move || Some(expected_config_dir));

            let expected_config_path: PathBuf = ["/some/config/dir", "user.json"].iter().collect();
            let rts_ctx = helpers::read_to_string_context();
            rts_ctx
                .expect()
                .with(eq(expected_config_path))
                .return_once(|_| Err(io::Error::from(io::ErrorKind::PermissionDenied)));

            assert_matches!(get_cli_credentials(),
                Err(Error::ConfigReadError(io_error)) if io_error.kind() == io::ErrorKind::PermissionDenied);
        }
    }
}
