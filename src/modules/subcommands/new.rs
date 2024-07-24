use std::process::Command;

use crate::{
    modules::{
        helpers::branch::{filter_branches_by_code, get_branches},
        structs::branch::Branch,
        types::errors::Error,
    },
    NewArgs,
};

pub fn exec(args: &NewArgs) -> Result<(), Error> {
    let branch_type = args.branch_type.as_str();
    let branch_code = args.branch_code.as_str();
    let branch_name = args.branch_name.as_str();
    let source = &args.source;
    let literal_source = args.literal_source;
    let from_current = args.from_current;
    let raw_name = Branch::make_raw_name(branch_type, branch_code, branch_name)?;
    let exit_code = if let Some(source) = source {
        let source = if literal_source || Branch::SPECIAL_NAMES.contains(&source.as_str()) {
            source.clone()
        } else {
            let branches = get_branches()?;
            let matches = filter_branches_by_code(&branches, source);
            if matches.len() == 0 {
                return Err(Error::BranchNotFoundOnCheckout(source.to_string()));
            }
            matches[0].clone()
        };
        Command::new("git")
            .arg("checkout")
            .arg("-b")
            .arg(&raw_name)
            .arg(source)
            .status()
            .map_err(|_| Error::Git)?
            .code()
            .unwrap_or(-1)
    } else if from_current {
        Command::new("git")
            .arg("checkout")
            .arg("-b")
            .arg(&raw_name)
            .status()
            .map_err(|_| Error::Git)?
            .code()
            .unwrap_or(-1)
    } else {
        let source_branch = match branch_type {
            "feature" => "develop",
            "hotfix" => "master", // ! needs configuring
            _ => return Err(Error::InvalidBranchType(branch_type.to_string())),
        };
        Command::new("git")
            .arg("checkout")
            .arg("-b")
            .arg(&raw_name)
            .arg(source_branch)
            .status()
            .map_err(|_| Error::Git)?
            .code()
            .unwrap_or(-1)
    };
    if exit_code != 0 {
        return Err(Error::Git);
    }
    Ok(())
}
