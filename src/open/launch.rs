use std::process::ExitStatus;
use std::{path::PathBuf, vec};

use anyhow::{anyhow, Result};
use http::Uri;

use crate::shared::platform::{Os, Platform};
use crate::shared::shutil::{look_path, run_subprocess};

/// Returns a vector list of commands required to 'open' for each OS, like `xdg-open` for Linux
fn get_os_specific_open_command(os: &Os) -> Vec<&'static str> {
    match os {
        Os::Windows => vec!["rundll32.exe", "url.dll,FileProtocolHandler"],
        Os::Linux => vec!["xdg-open"],
        Os::MacOS => vec!["open"],
    }
}

/// Returns an error if the specified command was not found by searching the path
fn ensure_command_available(cmd: &str) -> Result<()> {
    if look_path(cmd).is_none() {
        let msg = format!("{} is not installed", cmd);
        return Err(anyhow!(msg));
    }
    Ok(())
}

/// The opener can open a file, a url, etc.
pub enum Opener {
    Browser(Browser),
    File(File),
}

impl Opener {
    /// Open matches. For example, if the opener is a browser, it will open a url.
    pub fn open(&self) -> Result<ExitStatus> {
        match self {
            Opener::Browser(b) => b.open(),
            Opener::File(f) => f.open(),
        }
    }
}

pub trait Open {
    /// Open the file, url, etc.
    fn open(&self) -> Result<ExitStatus>;
}

pub struct Browser {
    uri: Uri,
    platform: Platform,
}

impl Browser {
    pub fn new(uri: Uri, platform: Platform) -> Self {
        Self { uri, platform }
    }

    pub fn from_string(uri: String, platform: Platform) -> Result<Self> {
        let uri = uri.parse::<Uri>()?;
        Ok(Self::new(uri, platform))
    }

    pub fn command(&self) -> Vec<&str> {
        get_os_specific_open_command(self.platform.os())
    }

    fn validate(&self) -> Result<()> {
        let os = self.platform.os();
        if os.eq(&Os::Linux) {
            let xdg_open = self.command()[0];
            ensure_command_available(xdg_open)?
        }
        Ok(())
    }
}

impl Open for Browser {
    fn open(&self) -> Result<ExitStatus> {
        self.validate()?;

        let mut list: Vec<&str> = vec![];
        let mut cmd = self.command();
        list.append(&mut cmd);

        let arg = self.uri.to_string();
        list.push(arg.as_str());
        match run_subprocess(list[0], &list[1..]) {
            Ok(status) => Ok(status),
            Err(_) => Err(anyhow!("Failed to open url")),
        }
    }
}

pub struct File {
    file: PathBuf,
    platform: Platform,
}

impl File {
    pub fn new(file: PathBuf, platform: Platform) -> Self {
        Self { file, platform }
    }

    pub fn command(&self) -> Vec<&str> {
        get_os_specific_open_command(self.platform.os())
    }

    fn validate(&self) -> Result<()> {
        let os = self.platform.os();
        if os.eq(&Os::Linux) {
            let xdg_open = self.command()[0];
            ensure_command_available(xdg_open)?
        }
        Ok(())
    }
}

impl Open for File {
    fn open(&self) -> Result<ExitStatus> {
        self.validate()?;

        let mut list: Vec<&str> = vec![];
        let mut cmd = self.command();
        list.append(&mut cmd);

        let arg = self.file.to_str().unwrap();
        list.push(arg);
        match run_subprocess(list[0], &list[1..]) {
            Ok(status) => Ok(status),
            Err(_) => {
                let msg = format!("Failed to open file: {}", self.file.display());
                Err(anyhow!(msg))
            }
        }
    }
}
