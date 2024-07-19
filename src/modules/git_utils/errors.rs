use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum BranchError {
    NameFormat,
    StringFormat,
    Git,
}

impl fmt::Display for BranchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BranchError::NameFormat => write!(f, "Branch name is in an invalid format."),
            BranchError::StringFormat => {
                write!(f, "Branch name has unsupported character or is corrupted.")
            }
            BranchError::Git => {
                write!(f, "Some error happened with git. Make sure git is installed correctly.")
            }
        }
    }
}
