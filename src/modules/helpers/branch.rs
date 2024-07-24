use std::process::Command;

use crate::modules::{structs::branch::Branch, types::errors::Error};

pub fn get_branches() -> Result<Vec<String>, Error> {
    let branches_output = Command::new("git")
        .arg("branch")
        .arg("-a")
        .output()
        .map_err(|_| Error::Git)?;
    if branches_output.status.code().unwrap_or(-1) != 0 {
        return Err(Error::Git);
    };
    let branches = String::from_utf8(branches_output.stdout).map_err(|_| Error::Git)?;
    let branches = branches
        .trim()
        .split("\n")
        .map(|branch| {
            let branch = branch.trim();
            if branch.starts_with("*") {
                (&branch[2..]).to_string()
            } else if branch.contains(" -> ") {
                let i = branch.find(" -> ").unwrap_or(branch.len());
                (&branch[0..i]).to_string()
            } else {
                branch.to_string()
            }
        })
        .collect::<Vec<String>>();
    Ok(branches)
}

pub fn filter_branches_by_code<'a>(branches: &'a Vec<String>, branch_code: &str) -> Vec<&'a String> {
    branches
        .iter()
        .filter(|branch_name| {
            if let Ok(branch) = Branch::from(&branch_name) {
                branch.code() == branch_code
            } else {
                false
            }
        })
        .collect::<Vec<&String>>()
}