use git2::{Error, Remote, Repository};

fn validate_remote_name(name: &str) -> Result<&str, Error> {
    let name = name.trim();
    if name.is_empty() {
        return Err(Error::from_str("remote name cannot be empty"));
    }
    if !Remote::is_valid_name(name) {
        return Err(Error::from_str("remote name is invalid"));
    }
    Ok(name)
}

fn validate_remote_url(url: &str) -> Result<&str, Error> {
    let url = url.trim();
    if url.is_empty() {
        return Err(Error::from_str("remote URL cannot be empty"));
    }
    Ok(url)
}

pub fn add_remote(repo: &Repository, name: &str, url: &str) -> Result<(), Error> {
    let name = validate_remote_name(name)?;
    let url = validate_remote_url(url)?;
    repo.remote(name, url)?;
    Ok(())
}

pub fn rename_remote(repo: &Repository, old_name: &str, new_name: &str) -> Result<(), Error> {
    let old_name = validate_remote_name(old_name)?;
    let new_name = validate_remote_name(new_name)?;
    if old_name == new_name {
        return Err(Error::from_str("new remote name must differ from current remote name"));
    }
    repo.remote_rename(old_name, new_name)?;
    Ok(())
}

pub fn delete_remote(repo: &Repository, name: &str) -> Result<(), Error> {
    let name = validate_remote_name(name)?;
    repo.remote_delete(name)?;
    Ok(())
}

pub fn set_remote_url(repo: &Repository, name: &str, url: &str) -> Result<(), Error> {
    let name = validate_remote_name(name)?;
    let url = validate_remote_url(url)?;
    repo.find_remote(name)?;
    repo.remote_set_url(name, url)?;
    Ok(())
}

pub fn set_remote_push_url(repo: &Repository, name: &str, push_url: Option<&str>) -> Result<(), Error> {
    let name = validate_remote_name(name)?;
    let push_url = push_url.map(str::trim).filter(|value| !value.is_empty());
    repo.find_remote(name)?;
    repo.remote_set_pushurl(name, push_url)?;
    Ok(())
}

#[cfg(test)]
#[path = "../../tests/git/actions/remotes.rs"]
mod tests;
