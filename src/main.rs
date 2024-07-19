mod modules;

use clap::{ArgAction, Args, Parser, Subcommand};
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

// ! chore, docs, feat, fix, refactor, style, or test should be added in the future
#[derive(Args)]
pub struct CommitArgs {
    #[clap(action, help="Can be either \"feat\", \"fix\", \"style\" or \"chore\"")]
    pub commit_type: String,

    #[clap(action)]
    pub message: String,

    #[clap(action=ArgAction::SetTrue, short, long, help="Run CI")]
    pub run_ci: bool,

    #[clap(action=ArgAction::SetFalse, short, long, help="Do not run git add before committing")]
    pub no_add: bool,

    #[clap(action=ArgAction::SetTrue, short, long, help="Allow empty commit")]
    pub empty: bool,
}

fn main() {
    let args = Cli::parse();

    match args.action {
        Action::Commit(args) => {
            commit(&args).unwrap_or_else(|error| panic!("\n{}", error.to_string()));
        }
    }
}

fn commit(args: &CommitArgs) -> Result<(), GitError> {
    let branch = Branch::new()?;
    let commit_type = match &args.commit_type[..] {
        "feat" => CommitType::Feat,
        "chore" => CommitType::Chore,
        "style" => CommitType::Style,
        "fix" => CommitType::Fix,
        _ => return Err(GitError::CommitType),
    };
    branch.commit(commit_type, &args.message, !args.no_add, args.empty, args.run_ci)?;
    Ok(())
}
