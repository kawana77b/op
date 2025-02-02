use std::path::{Path, PathBuf};

use http::Uri;
use regex::Regex;

fn is_probably_url(value: &str) -> bool {
    let re = Regex::new(r"^(https?://)?([\da-z\.-]+)\.([a-z\.]{2,6})([\/\w \.-]*)*\/?$").unwrap();
    re.is_match(value)
}

#[derive(Debug)]
pub enum ParamKind {
    FilePath(PathBuf),
    Url(Uri),
}

fn resolve_file(value: &str) -> Option<ParamKind> {
    let p = Path::new(value.trim());
    if !p.exists() {
        return None;
    }
    let mut dir = Some(p);
    if !p.is_dir() {
        dir = p.parent();
    }
    let d = dir?;
    if d.exists() {
        let full_path = std::path::absolute(d).unwrap();
        return Some(ParamKind::FilePath(full_path.to_owned()));
    }
    None
}

fn resolve_url(value: &str) -> Option<ParamKind> {
    if !is_probably_url(value) {
        return None;
    }
    let uri = value.trim().parse::<Uri>().ok()?;
    let scheme = uri.scheme_str()?;
    if scheme == "http" || scheme == "https" {
        return Some(ParamKind::Url(uri));
    }
    None
}

/// Identifies the type from the value and returns a type enumeration value wrapped in a dedicated value.
pub fn determine(value: &str) -> Option<ParamKind> {
    let f = resolve_file(value);
    if f.is_some() {
        return f;
    }
    let u = resolve_url(value);
    if u.is_some() {
        return u;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_file() {
        print!("{:?}", resolve_url("https://foobar.com"));
        assert_eq!(resolve_file("./").is_none(), false);
        assert_eq!(resolve_url("https://foobar.com").is_none(), false);
    }
}
