#[cfg(test)]
mod tests;

use std::process::Command;

use crate::modules::types::{commit_types::CommitType, errors::Error};

#[derive(PartialEq, Debug)]
pub struct Branch {
    branch_type: String,
    branch_code: String,
    branch_title: String,
    is_special: bool,
}

impl Branch {
    pub const VALID_TYPES: [&'static str; 2] = ["feature", "hotfix"];
    pub const SPECIAL_NAMES: [&'static str; 3] = ["develop", "main", "master"];
    pub const CODE_PREFIX: &'static str = "RCT-";
    pub const RUN_CI: &'static str = "(run_ci)";

    pub fn make_raw_name(
        branch_type: &str,
        branch_code: &str,
        branch_name: &str,
    ) -> Result<String, Error> {
        if Branch::SPECIAL_NAMES.contains(&branch_name)
            && branch_code.is_empty()
            && branch_type.is_empty()
        {
            return Ok(branch_name.to_string());
        }
        if !branch_code.bytes().all(|c| c.is_ascii_digit()) {
            return Err(Error::BranchCode);
        }
        if !Branch::VALID_TYPES.contains(&branch_type) {
            return Err(Error::InvalidBranchType(branch_type.to_string()));
        }
        let raw_name = format!(
            "{}/{}{}_{}",
            branch_type,
            Branch::CODE_PREFIX,
            branch_code,
            branch_name
        )
        .to_string();
        Ok(raw_name)
    }

    fn make_commit_message(&self, commit_type: CommitType, message: &str, run_ci: bool) -> String {
        let commit_type = match commit_type {
            CommitType::Chore => "chore",
            CommitType::Feat => "feat",
            CommitType::Fix => "fix",
            CommitType::Style => "style",
        };
        if self.is_special {
            format!(
                "{}: {}{}",
                commit_type,
                message,
                if run_ci {
                    " ".to_string() + Self::RUN_CI
                } else {
                    "".to_string()
                }
            )
        } else {
            format!(
                "{}(RCT-{}): {}{}",
                commit_type,
                self.branch_code,
                message.trim(),
                if run_ci {
                    format!(" {}", Self::RUN_CI)
                } else {
                    "".to_string()
                }
            )
        }
    }

    pub fn commit(
        &self,
        commit_type: CommitType,
        message: &str,
        add: bool,
        allow_empty: bool,
        run_ci: bool,
    ) -> Result<(), Error> {
        let commit_message = self.make_commit_message(commit_type, message, run_ci);
        if add {
            let exit_code = Command::new("git")
                .arg("add")
                .arg(".")
                .status()
                .map_err(|_| Error::Git)?
                .code()
                .unwrap_or(-1);
            if exit_code != 0 {
                return Err(Error::Add);
            }
        }
        let exit_code = if allow_empty {
            Command::new("git")
                .arg("commit")
                .arg("--allow-empty")
                .arg("-m")
                .arg(&commit_message)
                .status()
                .map_err(|_| Error::Git)?
                .code()
                .unwrap_or(-1)
        } else {
            Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg(&commit_message)
                .status()
                .map_err(|_| Error::Git)?
                .code()
                .unwrap_or(-1)
        };
        if exit_code != 0 {
            Err(Error::Commit)
        } else {
            Ok(())
        }
    }

    pub fn new() -> Result<Self, Error> {
        let raw_name = Self::get_raw_name()?;
        let (branch_type, branch_code, branch_title, is_special) = Self::parse_name(&raw_name)?;
        return Ok(Branch {
            branch_code,
            branch_title,
            branch_type,
            is_special,
        });
    }

    pub fn from(name: &str) -> Result<Self, Error> {
        let (branch_type, branch_code, branch_title, is_special) = Self::parse_name(&name)?;
        return Ok(Branch {
            branch_code,
            branch_title,
            branch_type,
            is_special,
        });
    }

    pub fn code(&self) -> &str {
        &self.branch_code
    }

    fn parse_name(name: &str) -> Result<(String, String, String, bool), Error> {
        let name = name.trim();
        if !Self::validate_name(name) {
            return Err(Error::NameFormat);
        }
        if Self::SPECIAL_NAMES.contains(&name) {
            return Ok((String::from(""), String::from(""), String::from(name), true));
        };
        let slash_index = name.find('/').ok_or(Error::NameFormat)?;
        let branch_code_index =
            name.find(Self::CODE_PREFIX).ok_or(Error::NameFormat)? + Self::CODE_PREFIX.len();
        let branch_code_index_end = name.find('_').ok_or(Error::NameFormat)?;
        let branch_title_index = branch_code_index_end + 1;
        let branch_type = String::from(&name[..slash_index]);
        let branch_code = String::from(&name[branch_code_index..branch_code_index_end]);
        let branch_title = String::from(&name[branch_title_index..]);
        return Ok((branch_type, branch_code, branch_title, false));
    }

    fn validate_name(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        if Self::SPECIAL_NAMES.contains(&name) {
            return true;
        };
        'outer: for valid_type in Self::VALID_TYPES.iter() {
            if !name.starts_with(&format!("{}/{}", valid_type, Self::CODE_PREFIX)) {
                continue;
            };
            let branch_code_start = valid_type.len() + Self::CODE_PREFIX.len() + 1;

            for (i, c) in name[branch_code_start..].chars().enumerate() {
                if i == 0 && (c < '0' || c > '9') || i == name.len() - branch_code_start - 1 {
                    continue 'outer;
                }
                if c >= '0' && c <= '9' {
                    continue;
                }
                if c == '_' {
                    break;
                }
            }
            return true;
        }
        return false;
    }

    fn get_raw_name() -> Result<String, Error> {
        Command::new("git")
            .arg("branch")
            .arg("--show-current")
            .output()
            .map_err(|_| Error::Git)
            .and_then(|output| {
                if output.status.code().unwrap_or(-1) == 0 {
                    String::from_utf8(output.stdout)
                        .map_err(|_| Error::StringFormat)
                        .map(|name| name.trim().to_string())
                } else {
                    Err(Error::Git)
                }
            })
    }
}
