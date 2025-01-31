use std::{env, path::Path, vec};

use anyhow::{anyhow, Error, Result};
use regex::Regex;

use crate::shared::shutil;

fn check_if_git_directory(path: &str) -> Result<()> {
    let git_dir = Path::new(path).join(".git");
    match git_dir.is_dir() {
        true => Ok(()),
        false => Err(anyhow!("Not a git project directory")),
    }
}

fn check_git_availability() -> Result<()> {
    match shutil::look_path("git") {
        Some(_) => Ok(()),
        None => Err(anyhow!("git is not installed")),
    }
}

fn is_ssh(url: &str) -> bool {
    let re = Regex::new(r"^git@.+:.+.git$").unwrap();
    re.is_match(url)
}

fn get_cmd_output(cmd: &str, args: &[&str]) -> Result<String, Error> {
    let exec_result = shutil::run_subprocess_ex(cmd, args).and_then(|output| {
        String::from_utf8(output.stdout).or(Err(anyhow!("Failed to get command result")))
    });
    let raw = exec_result?;
    Ok(raw)
}

/// Format the url to like https://github.com. If it is ssh, convert it to https.
fn format_url(url: &str) -> String {
    let mut u = String::from(url.trim());
    if is_ssh(u.as_str()) {
        // git@github.com:xxxxxx/xxxxxxxxxx.git
        let sshexp = Regex::new(r"^git@").unwrap();
        u = sshexp.replace_all(&u, "").replacen(':', "/", 1);
        u = format!("https://{}", u);
    }
    // Remove the xxxxxx.git and trim
    u.trim_end_matches(".git").to_string()
}

/// Get the remote url of the git repository.
pub fn get_remote_url(dir: &str) -> Result<String> {
    if env::set_current_dir(dir).is_err() {
        return Err(anyhow!("Failed to change directory"));
    }
    check_if_git_directory(dir)?;
    check_git_availability()?;
    let output = get_cmd_output("git", &vec!["config", "--get", "remote.origin.url"])?;
    let trimmed = output.trim();
    if trimmed.len() == 0 {
        return Err(anyhow!("Failed to get remote url"));
    }
    Ok(format_url(trimmed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ssh() {
        assert_eq!(is_ssh("git@github.com:foo/bar.git"), true);
        assert_eq!(is_ssh("git@github.com:foo/bar"), false);
    }

    #[test]
    fn test_format_url() {
        let ssh_url = "git@github.com:foo/bar.git";
        let ssh_formatted = format_url(ssh_url);
        assert_eq!("https://github.com/foo/bar", ssh_formatted);

        let http_url = "https://github.com/foo/bar.git";
        let http_formatted = format_url(http_url);
        assert_eq!("https://github.com/foo/bar", http_formatted);
    }
}
