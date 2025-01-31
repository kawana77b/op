use std::process::ExitCode;

mod cmd;
mod open;
mod shared;

fn main() -> ExitCode {
    match cmd::root::execute() {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
