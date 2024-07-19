use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum GitError {
    NameFormat,
    StringFormat,
    BranchCode,
    Git,
    CommitType,
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
            GitError::BranchCode => {
                write!(f, "Branch code is invalid.")
            }
            GitError::CommitType => {
                write!(f, "Invalid commit type.")
            }
        }
    }
}
