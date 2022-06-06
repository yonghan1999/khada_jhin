use crate::core::error::failed_to_open_lock_file_error::FailedToOpenLockfileError;
use crate::core::error::initialization_error::InitializationError;
use crate::core::error::lock_file_error::{LockFileError, LockfileErrorTypes};
use std::fs::read_to_string;
use std::process::{Command, Stdio};

use encoding::{Encoding, DecoderTrap};
use encoding::all::UTF_8;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REMOTING_AUTH_TOKEN: Regex = Regex::new("--remoting-auth-token=([\\w-]*)").unwrap();
    static ref APP_PORT: Regex = Regex::new("--app-port=([0-9]*)").unwrap();
}

#[derive(Debug, Clone)]
pub struct LockfileContents {
    pub port: u16,
    pub password: String,
    pub protocol: String,
}

pub fn get_lockfile() -> Result<LockfileContents, InitializationError> {
    return if cfg!(windows) {
        let out = Command::new("WMIC")
            .args([
                "PROCESS",
                "WHERE",
                "name='LeagueClientUx.exe'",
                "GET",
                "commandline",
            ])
            .stdout(Stdio::piped())
            .output()
            .unwrap();
        let out_string = UTF_8.decode(&out.stdout, DecoderTrap::Ignore).unwrap();
        let mut stdout = out_string
            .lines()
            .filter(|line| line.find("LeagueClientUx").is_some());
        let first_line = stdout.next().unwrap_or("");
        if !first_line.contains("LeagueClientUx") {
            return Err(InitializationError::GenericLockfile(LockFileError::new(
                LockfileErrorTypes::LeagueNotRunning,
            )));
        }
        match REMOTING_AUTH_TOKEN.find(first_line) {
            Some(remote_auth) => match APP_PORT.find(first_line) {
                Some(port) => Ok(LockfileContents {
                    port: port.as_str().split("=").collect::<Vec<&str>>()[1]
                        .parse::<u16>()
                        .unwrap(),
                    password: remote_auth.as_str().split("=").collect::<Vec<&str>>()[1].to_string(),
                    protocol: "https".to_string(),
                }),
                None => Err(InitializationError::GenericLockfile(LockFileError::new(
                    LockfileErrorTypes::LeagueNotRunning,
                ))),
            },
            None => Err(InitializationError::GenericLockfile(LockFileError::new(
                LockfileErrorTypes::LeagueNotRunning,
            ))),
        }
    } else {
        if cfg!(target_os = "macos") {
            let out = Command::new("ps")
                .args(["-A"])
                .stdout(Stdio::piped())
                .output()
                .unwrap();
            let out_string = String::from_utf8(out.stdout).unwrap();
            let mut stdout = out_string
                .lines()
                .filter(|line| line.find("LeagueClientUx").is_some());
            let first_line = stdout.next().unwrap_or("");
            if !first_line.contains("LeagueClientUx") {
                return Err(InitializationError::GenericLockfile(LockFileError::new(
                    LockfileErrorTypes::LeagueNotRunning,
                )));
            }
            match REMOTING_AUTH_TOKEN.find(first_line) {
                Some(remote_auth) => match APP_PORT.find(first_line) {
                    Some(port) => Ok(LockfileContents {
                        port: port.as_str().split("=").collect::<Vec<&str>>()[1]
                            .parse::<u16>()
                            .unwrap(),
                        password: remote_auth.as_str().split("=").collect::<Vec<&str>>()[1]
                            .to_string(),
                        protocol: "https".to_string(),
                    }),
                    None => Err(InitializationError::GenericLockfile(LockFileError::new(
                        LockfileErrorTypes::LeagueNotRunning,
                    ))),
                },
                None => Err(InitializationError::GenericLockfile(LockFileError::new(
                    LockfileErrorTypes::LeagueNotRunning,
                ))),
            }
        } else {
            Err(InitializationError::UnknownError(
                "Unsupported platform.".to_string(),
            ))
        }
    };
}
