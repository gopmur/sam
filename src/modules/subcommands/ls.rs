use crate::modules::{helpers::branch::get_branches, types::errors::Error};

pub fn exec() -> Result<(), Error> {
    let mut branches = get_branches()?;
    branches.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    for branch in branches {
        println!("{}", branch);
    }
    Ok(())
}
