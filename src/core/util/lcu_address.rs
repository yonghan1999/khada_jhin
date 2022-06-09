use std::process::{Command, Stdio};

use encoding::all::UTF_8;
use encoding::{DecoderTrap, Encoding};
use lazy_static::lazy_static;
use regex::Regex;

use crate::core::error::initialization_error::InitializationError;
use crate::core::error::lcu_address_error::{LcuAddressError, LcuAddressErrorTypes};

lazy_static! {
    static ref REMOTING_AUTH_TOKEN: Regex = Regex::new("--remoting-auth-token=([\\w-]*)").unwrap();
    static ref APP_PORT: Regex = Regex::new("--app-port=([0-9]*)").unwrap();
}

#[derive(Debug, Clone)]
pub struct LcuAddressContents {
    pub port: u16,
    pub password: String,
    pub protocol: String,
}

/// 获取 LCU 的 remoting-auth-token 和 app-port。
/// ！！！暂未适配macOS
pub fn get_lcu_address() -> Result<LcuAddressContents, InitializationError> {
    return if cfg!(windows) {
        // 运行命令来查看 LeagueClientUx.exe 的参数，从而找到 remoting-auth-token 和 app-port
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
        // 将命令的 stdout 按 UTF-8 转换为 String
        let out_string = UTF_8.decode(&out.stdout, DecoderTrap::Ignore).unwrap();
        // 从标准输出中解析出 LcuAddressContents
        parse_address(out_string)
    } else {
        if cfg!(target_os = "macos") {
            let out = Command::new("ps")
                .args(["-A"])
                .stdout(Stdio::piped())
                .output()
                .unwrap();
            let out_string = String::from_utf8(out.stdout).unwrap();
            parse_address(out_string)
        } else {
            Err(InitializationError::UnknownError(
                "Unsupported platform.".to_string(),
            ))
        }
    };
}

fn parse_address(out_string: String)  -> Result<LcuAddressContents, InitializationError> {
    let mut stdout = out_string
        .lines()
        .filter(|line| line.find("LeagueClientUx").is_some());
    let first_line = stdout.next().unwrap_or("");
    if !first_line.contains("LeagueClientUx") {
        return Err(InitializationError::CommandNotFound(LcuAddressError::new(
            LcuAddressErrorTypes::LeagueNotRunning,
        )));
    }
    match REMOTING_AUTH_TOKEN.find(first_line) {
        Some(remote_auth) => match APP_PORT.find(first_line) {
            Some(port) => Ok(LcuAddressContents {
                port: port.as_str().split("=").collect::<Vec<&str>>()[1]
                    .parse::<u16>()
                    .unwrap(),
                password: remote_auth.as_str().split("=").collect::<Vec<&str>>()[1].to_string(),
                protocol: "https".to_string(),
            }),
            None => Err(InitializationError::CommandNotFound(LcuAddressError::new(
                LcuAddressErrorTypes::LeagueNotRunning,
            ))),
        },
        None => Err(InitializationError::CommandNotFound(LcuAddressError::new(
            LcuAddressErrorTypes::LeagueNotRunning,
        ))),
    }
}
