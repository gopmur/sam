use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum GitError {
    NameFormat,
    StringFormat,
    Add,
    Git,
    CommitType,
    Commit,
    BranchCode,
    BranchNotFoundOnCheckout(String),
    InvalidBranchType(String),
    IncompatibleArguments(Vec<String>),
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitError::NameFormat => write!(f, "Branch name is in an invalid format."),
            GitError::StringFormat => {
                write!(f, "Branch name has unsupported character or is corrupted.")
            }
            GitError::Git => {
                write!(
                    f,
                    "Some error happened with git. Make sure git is installed correctly."
                )
            }
            GitError::Add => {
                write!(f, "Some error occurred while running git add.")
            }
            GitError::CommitType => {
                write!(f, "Invalid commit type.")
            }
            GitError::Commit => {
                write!(f, "Some error happened while committing.")
            }
            GitError::BranchCode => {
                write!(f, "Invalid branch code.")
            }
            GitError::BranchNotFoundOnCheckout(branch_code) => {
                write!(f, "Branch with the code {} does not exists", branch_code)
            }
            GitError::InvalidBranchType(branch_type) => {
                write!(f, "Branch type {} is invalid", branch_type)
            }
            GitError::IncompatibleArguments(arguments) => {
                write!(f, "Incompatible arguments: {}", arguments.join(", "))
            }
        }
    }
}
