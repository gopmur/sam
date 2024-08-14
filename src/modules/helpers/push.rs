use std::process::Command;

use crate::modules::types::errors::Error;

pub fn git_push() -> Result<(), Error> {
    let status = Command::new("git")
        .arg("push")
        .arg("-u")
        .arg("origin")
        .arg("HEAD")
        .status()
        .map_err(|_| Error::Git)?;
    if status.code().unwrap_or(-1) != 0 {
        return Err(Error::Git);
    }
    Ok(())
}
