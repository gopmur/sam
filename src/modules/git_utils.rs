pub mod errors;
#[cfg(test)]
mod tests;

use errors::GitError;
use std::process::Command;

pub enum CommitType {
    Feat,
    Chore,
    Style,
    Fix,
}

#[derive(PartialEq, Debug)]
pub struct Branch {
    branch_type: String,
    branch_code: String,
    branch_title: String,
    is_special: bool,
}

impl Branch {
    const VALID_TYPES: [&'static str; 2] = ["feature", "hotfix"];
    const SPECIAL_NAMES: [&'static str; 3] = ["develop", "main", "master"];
    const CODE_PREFIX: &'static str = "RCT-";

    fn make_commit_message(&self, commit_type: CommitType, message: &str) -> String {
        let commit_type = match commit_type {
            CommitType::Chore => "chore",
            CommitType::Feat => "feat",
            CommitType::Fix => "fix",
            CommitType::Style => "style",
        };
        if self.is_special {
            format!("{}: {}", commit_type, message)
        } else {
            format!(
                "{}(RCT-{}): {}",
                commit_type,
                self.branch_code,
                message.trim()
            )
        }
    }

    pub fn commit(
        &self,
        commit_type: CommitType,
        message: &str,
        add: bool,
        allow_empty: bool,
    ) -> Result<(), GitError> {
        let commit_message = self.make_commit_message(commit_type, message);
        if add {
            let exit_code = Command::new("git")
                .arg("add")
                .arg(".")
                .status()
                .map_err(|_| GitError::Git)?
                .code()
                .unwrap_or(-1);
            if exit_code != 0 {
                return Err(GitError::Add);
            }
        }
        let exit_code = if allow_empty {
            Command::new("git")
                .arg("commit")
                .arg("--allow-empty")
                .arg("-m")
                .arg(&commit_message)
                .status()
                .map_err(|_| GitError::Git)?
                .code()
                .unwrap_or(-1)
        } else {
            Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg(&commit_message)
                .status()
                .map_err(|_| GitError::Git)?
                .code()
                .unwrap_or(-1)
        };
        if exit_code != 0 {
            Err(GitError::Commit)
        } else {
            Ok(())
        }
    }

    pub fn new() -> Result<Self, GitError> {
        let raw_name = Self::get_raw_name()?;
        let (branch_type, branch_code, branch_title, is_special) = Self::parse_name(&raw_name)?;
        return Ok(Branch {
            branch_code,
            branch_title,
            branch_type,
            is_special,
        });
    }

    fn parse_name(name: &str) -> Result<(String, String, String, bool), GitError> {
        if !Self::validate_name(name) {
            return Err(GitError::NameFormat);
        }
        if Self::SPECIAL_NAMES.contains(&name) {
            return Ok((String::from(""), String::from(""), String::from(name), true));
        };
        let slash_index = name.find('/').ok_or(GitError::NameFormat)?;
        let branch_code_index =
            name.find(Self::CODE_PREFIX).ok_or(GitError::NameFormat)? + Self::CODE_PREFIX.len();
        let branch_code_index_end = name.find('_').ok_or(GitError::NameFormat)?;
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

    fn get_raw_name() -> Result<String, GitError> {
        Command::new("git")
            .arg("branch")
            .arg("--show-current")
            .output()
            .map_err(|_| GitError::Git)
            .and_then(|output| {
                if output.status.code().unwrap_or(-1) == 0 {
                    String::from_utf8(output.stdout)
                        .map_err(|_| GitError::StringFormat)
                        .map(|name| name.trim().to_string())
                } else {
                    Err(GitError::Git)
                }
            })
    }
}
