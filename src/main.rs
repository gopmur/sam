mod modules;

use clap::{ArgAction, Args, Parser, Subcommand};
use modules::subcommands;
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

    // ! FIX source should use branch_code
    #[clap(action, short, long, conflicts_with = "from_current")]
    pub source: Option<String>,

    #[clap(action=ArgAction::SetTrue, short, long, requires = "source")]
    pub literal_source: bool,

    #[clap(action=ArgAction::SetTrue, short='c', long, help="Create branch from current branch", conflicts_with="source")]
    pub from_current: bool,
    // TODO add --force option for duplicate code
}

#[derive(Args)]
pub struct CheckoutArgs {
    #[clap(action)]
    pub branch_code: String,
    // TODO add --literal option
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
        Action::Commit(args) => subcommands::commit::exec(&args),
        Action::Checkout(args) => subcommands::checkout::exec(&args.branch_code),
        // TODO check for duplicate branch code
        Action::New(args) => subcommands::new::exec(&args),
    }
    .unwrap_or_else(|error| {
        println!("{}", error);
        process::exit(-1);
    })
}
