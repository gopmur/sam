use crate::modules::{
    helpers::push::git_push,
    structs::branch::Branch,
    types::{commit_types::CommitType, errors::Error},
};

pub fn exec() -> Result<(), Error> {
    let branch = Branch::new()?;
    branch.commit(CommitType::Chore, Branch::RUN_CI, true, true, false)?;
    git_push()?;
    Ok(())
}
