use anyhow::{anyhow, Result};
use clap::Args;

use crate::open::{
    git::get_remote_url,
    launch::{Browser, Open},
};
use crate::shared::platform::Platform;
use crate::shared::shutil::cwd;

#[derive(Args, Clone, Debug)]
pub struct GitArgs {}

pub fn run(args: &GitArgs) -> Result<()> {
    _ = args;
    let arg = cwd();
    if arg.is_none() {
        return Err(anyhow!("Argument parsing failed."));
    }
    let platform: Platform = Platform::get()?;
    let url = get_remote_url(arg.unwrap().as_str())?;
    let browser = Browser::from_string(url, platform)?;
    match browser.open() {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
