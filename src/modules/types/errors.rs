use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NameFormat,
    StringFormat,
    Add,
    Git,
    CommitType,
    Commit,
    BranchCode,
    BranchNotFoundOnCheckout(String),
    InvalidBranchType(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NameFormat => write!(f, "Branch name is in an invalid format."),
            Error::StringFormat => {
                write!(f, "Branch name has unsupported character or is corrupted.")
            }
            Error::Git => {
                write!(
                    f,
                    "Some error happened with git. Make sure git is installed correctly."
                )
            }
            Error::Add => {
                write!(f, "Some error occurred while running git add.")
            }
            Error::CommitType => {
                write!(f, "Invalid commit type.")
            }
            Error::Commit => {
                write!(f, "Some error happened while committing.")
            }
            Error::BranchCode => {
                write!(f, "Invalid branch code.")
            }
            Error::BranchNotFoundOnCheckout(branch_code) => {
                write!(f, "Branch with the code {} does not exists", branch_code)
            }
            Error::InvalidBranchType(branch_type) => {
                write!(f, "Branch type {} is invalid", branch_type)
            }
        }
    }
}
