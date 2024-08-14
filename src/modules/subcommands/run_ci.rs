use crate::modules::{
    helpers::push::git_push,
    structs::branch::Branch,
    types::{commit_types::CommitType, errors::Error},
};

pub fn exec() -> Result<(), Error> {
    let branch = Branch::new()?;
    // 08 is the ascii character for backspace and is needed
    // here because run_ci will add a redundant space
    // we can also just pass (run_ci) to message
    // but I did't do that because the string for
    // communicating the meaning of run_ci can
    // change
    let bs = vec![8];
    // this will never fail because 08 is a valid ascii
    let message = String::from_utf8(bs).unwrap();
    branch.commit(CommitType::Chore, &message, true, true, true)?;
    git_push()?;
    Ok(())
}
