mod modules;

use clap::{ArgAction, Args, Parser, Subcommand};
use modules::git_utils::{self, errors::GitError, Branch, CommitType};
use std::process;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    Commit(CommitArgs),
    Checkout(CheckoutArgs),
    New(NewArgs),
}

#[derive(Args)]
pub struct NewArgs {
    #[clap(action)]
    pub branch_type: String,

    #[clap(action)]
    pub branch_code: String,

    #[clap(action)]
    pub branch_name: String,

    #[clap(action, short, long, conflicts_with = "from_current")]
    pub source: Option<String>,

    #[clap(action=ArgAction::SetTrue, short='c', long, help="Create branch from current branch", conflicts_with="source")]
    pub from_current: bool,
}

#[derive(Args)]
pub struct CheckoutArgs {
    #[clap(action)]
    pub branch_code: String,
}

// ! chore, docs, feat, fix, refactor, style, or test should be added in the future
#[derive(Args)]
pub struct CommitArgs {
    #[clap(
        action,
        help = "Can be either \"feat\", \"fix\", \"style\" or \"chore\""
    )]
    pub commit_type: String,

    #[clap(action)]
    pub message: String,

    #[clap(action=ArgAction::SetTrue, short, long, help="Run CI")]
    pub run_ci: bool,

    #[clap(action=ArgAction::SetTrue, short, long, help="Do not run git add before committing")]
    pub no_add: bool,

    #[clap(action=ArgAction::SetTrue, short, long, help="Allow empty commit")]
    pub empty: bool,
}

fn main() {
    let args = Cli::parse();

    match args.action {
        Action::Commit(args) => Sam::commit(&args),
        Action::Checkout(args) => Sam::checkout(&args),
        Action::New(args) => Sam::new(&args),
    }
    .unwrap_or_else(|error| {
        println!("{}", error);
        process::exit(-1);
    })
}

struct Sam;
impl Sam {
    fn commit(args: &CommitArgs) -> Result<(), GitError> {
        let branch = Branch::new()?;
        let commit_type = match &args.commit_type[..] {
            "feat" => CommitType::Feat,
            "chore" => CommitType::Chore,
            "style" => CommitType::Style,
            "fix" => CommitType::Fix,
            _ => return Err(GitError::CommitType),
        };
        branch.commit(
            commit_type,
            &args.message,
            !args.no_add,
            args.empty,
            args.run_ci,
        )?;
        Ok(())
    }

    fn checkout(args: &CheckoutArgs) -> Result<(), GitError> {
        git_utils::checkout(&args.branch_code)?;
        Ok(())
    }

    // TODO check for duplicate branch code
    fn new(args: &NewArgs) -> Result<(), GitError> {
        git_utils::new_branch(
            &args.branch_type,
            &args.branch_code,
            &args.branch_name,
            &args.source,
            args.from_current,
        )?;
        Ok(())
    }
}
