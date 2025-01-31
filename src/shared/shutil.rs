use anyhow::{anyhow, Result};

use super::platform::Platform;
use std::process::{Command, ExitStatus, Output, Stdio};

/// Retrive the current directory path
pub fn cwd() -> Option<String> {
    let cwd = std::env::current_dir().ok()?;
    Some(cwd.display().to_string())
}

/// Returns the full path of the command.
pub fn look_path(cmd: &str) -> Option<String> {
    let plat = Platform::get().ok()?;
    let paths = std::env::var("PATH").ok()?;
    let separator = plat.os().path_sep();
    for path in paths.split(separator) {
        let path = path.trim();
        if path.is_empty() {
            continue;
        }
        let full_path = std::path::Path::new(path).join(cmd);
        if full_path.is_file() {
            let result = full_path.to_string_lossy().to_string();
            return Some(result);
        }
    }
    None
}

/// Execute a subprocess and return the exit status.
pub fn run_subprocess(cmd: &str, args: &[&str]) -> Result<ExitStatus> {
    let result = Command::new(cmd).args(args).spawn()?.wait();
    match result {
        Ok(status) => Ok(status),
        Err(e) => Err(anyhow!(format!("{}", e))),
    }
}

/// Execute a subprocess and return the output info.
pub fn run_subprocess_ex(cmd: &str, args: &[&str]) -> Result<Output> {
    let result = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();
    match result {
        Ok(output) => Ok(output),
        Err(e) => Err(anyhow!(format!("{}", e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subprocess() {
        let plat = Platform::get();
        if plat.is_err() {
            return;
        }
        if let Ok(plat) = plat {
            // Linux / MacOSなら以下のテストケースはOKなはず. Windowsは除外
            if plat.os().to_string().as_str() == "windows" {
                return;
            }
            let list = vec!["ls", "./"];
            match run_subprocess(list[0], &list[1..]) {
                Ok(s) => assert_eq!(s.success(), true),
                Err(_) => panic!(),
            }
        }
    }
}
