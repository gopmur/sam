mod modules;

use clap::{Args, Parser, Subcommand};
use modules::git_utils::{errors::GitError, Branch, CommitType};

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
    pub commit_type: String,

    #[clap(action)]
    pub message: String,
}

fn main() {
    let args = Cli::parse();

    match args.action {
        Action::Commit(commit_args) => {
            commit(&commit_args.commit_type, &commit_args.message).unwrap();
        }
    }
}

fn commit(commit_type: &str, message: &str) -> Result<(), GitError> {
    let branch = Branch::new()?;
    let commit_type = match commit_type {
        "feat" => CommitType::Feat,
        "chore" => CommitType::Chore,
        "style" => CommitType::Style,
        "fix" => CommitType::Fix,
        _ => return Err(GitError::CommitType),
    };
    branch.commit(commit_type, message, false)?;
    Ok(())
}
