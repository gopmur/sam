use crate::{
    modules::{
        structs::branch::Branch,
        types::{commit_types::CommitType, errors::Error},
    },
    CommitArgs,
};

pub fn exec(args: &CommitArgs) -> Result<(), Error> {
    let branch = Branch::new()?;
    let commit_type = match &args.commit_type[..] {
        "feat" => CommitType::Feat,
        "chore" => CommitType::Chore,
        "style" => CommitType::Style,
        "fix" => CommitType::Fix,
        _ => return Err(Error::CommitType),
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
