use anyhow::anyhow;
use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::cmd::git;
use crate::open::kind::{determine, ParamKind};
use crate::open::launch::{Browser, File, Opener};
use crate::shared::platform::Platform;
use crate::shared::shutil::cwd;

struct CommandInfo {
    name: Option<&'static str>,
    about: &'static str,
    long_about: Option<&'static str>,
}

struct CommandStructure {
    root: CommandInfo,
    git: CommandInfo,
}

const STRUCTURE: CommandStructure = CommandStructure {
  root: CommandInfo{
    name: None,
    about: "Open the file path or web address in the prescribed file explorer or browser",
    long_about: Some(r"Open the file path or web address in the prescribed file explorer or browser  

- This command works only on Windows, Mac, and Linux.
- If no arguments are given, it opens the current directory with the specified filer.
- If a file path is given, the directory will be opened with the specified filer."
)},
git: CommandInfo{
    name: Some("git"),
    about: "Open the git remote repository in the current directory or the directory given as argument in a browser",
    long_about: None
  },
};

#[derive(Parser, Debug)]
#[command(version, about = STRUCTURE.root.about, long_about = STRUCTURE.root.long_about)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Commands>,
    file: Option<String>,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    #[command(
        name = STRUCTURE.git.name.unwrap(),
        about = STRUCTURE.git.about,
    )]
    Git(git::GitArgs),
}

/// Execute the command.
pub fn execute() -> Result<()> {
    pre_run()?;
    let cli = Cli::parse();
    let result: Result<()> = match &cli.cmd {
        Some(subcommand) => run_subcommand(subcommand),
        _ => run(&cli),
    };
    match result {
        Ok(v) => Ok(v),
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

fn run_subcommand(subcommand: &Commands) -> Result<()> {
    match subcommand {
        Commands::Git(args) => git::run(args),
    }
}

fn run(args: &Cli) -> Result<()> {
    let arg = args.file.to_owned().or(cwd());
    if arg.is_none() {
        return Err(anyhow!("Argument parsing failed."));
    }
    let kind = determine(arg.unwrap().as_str());
    if kind.is_none() {
        return Err(anyhow!("Invalid argument value."));
    }
    let plat = Platform::get()?;
    let opener = match kind.unwrap() {
        ParamKind::FilePath(f) => Opener::File(File::new(f, plat)),
        ParamKind::Url(u) => Opener::Browser(Browser::new(u, plat)),
    };
    match opener.open() {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }
}

fn pre_run() -> Result<()> {
    // Check if the platform is supported (Windows, Mac, Linux is OK)
    match Platform::get() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!(
                "Sorry, this tool is not compatible with the OS of the machine it is running on."
            );
            Err(e)
        }
    }
}
