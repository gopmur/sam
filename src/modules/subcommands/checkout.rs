use std::process::Command;

use crate::modules::{helpers::branch::{filter_branches_by_code, get_branches}, structs::branch::Branch, types::errors::Error};

pub fn exec(input: &str) -> Result<(), Error> {
    if Branch::SPECIAL_NAMES.contains(&input) {
        checkout_literal(input)?;
        return Ok(());
    }
    checkout_with_code(input)?;
    Ok(())
}

fn checkout_with_code(branch_code: &str) -> Result<(), Error> {
    if !branch_code.bytes().all(|c| c.is_ascii_digit()) {
        return Err(Error::BranchCode);
    };
    let branches = get_branches()?;
    let matches = filter_branches_by_code(&branches, branch_code);
    if matches.is_empty() {
        return Err(Error::BranchNotFoundOnCheckout(branch_code.to_string()));
    }
    let first_match = &matches[0];
    checkout_literal(&first_match)?;
    Ok(())
}

fn checkout_literal(branch_name: &str) -> Result<(), Error> {
    let exit_code = Command::new("git")
        .arg("checkout")
        .arg(branch_name)
        .status()
        .map_err(|_| Error::Git)?
        .code()
        .unwrap_or(-1);
    if exit_code != 0 {
        return Err(Error::Git);
    }
    Ok(())
}
