mod modules;

use clap::{Args, Parser, Subcommand};
use modules::git_utils;
use std::process::Command;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    Commit(CommitArgs),
}

#[derive(Args)]
pub struct CommitArgs {
    #[clap(action)]
    pub name: String,
}

fn main() {
    let args = Cli::parse();
    match args.action {
        _ => {}
    }
}
