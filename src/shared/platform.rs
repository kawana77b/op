use std::str::FromStr;

use anyhow::Result;
use strum_macros::{Display, EnumString};

#[derive(PartialEq, EnumString, Display)]
pub enum Os {
    #[strum(serialize = "windows")]
    Windows,
    #[strum(serialize = "linux")]
    Linux,
    #[strum(serialize = "macos")]
    MacOS,
}

impl Os {
    /// Get the PATH separator for the current platform.
    pub fn path_sep(&self) -> &'static str {
        match self {
            Os::Windows => ";",
            _ => ":",
        }
    }
}

pub struct Platform {
    os: Os,
}

impl Platform {
    fn new(os: Os) -> Self {
        Self { os }
    }

    /// Get the current platform. If the platform is not supported, return an error.
    pub fn get() -> Result<Self> {
        match Os::from_str(std::env::consts::OS) {
            Ok(os) => Ok(Platform::new(os)),
            Err(_) => Err(anyhow::anyhow!("Unsupported platform")),
        }
    }

    pub fn os(&self) -> &Os {
        &self.os
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_ok() {
        assert_eq!(Os::from_str("windows").unwrap().eq(&Os::Windows), true);
        assert_eq!(Os::from_str("linux").unwrap().eq(&Os::Linux), true);
        assert_eq!(Os::from_str("macos").unwrap().eq(&Os::MacOS), true);

        assert_eq!(Os::Windows.to_string().as_str(), "windows");
    }

    #[test]
    fn test_path_sep() {
        assert_eq!(Os::Windows.path_sep(), ";");
        assert_eq!(Os::Linux.path_sep(), ":");
    }

    #[test]
    fn test_os_not_supported_now() {
        assert_eq!(Os::from_str("haiku").is_err(), true);
    }
}
