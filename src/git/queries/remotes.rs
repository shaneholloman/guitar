use git2::Repository;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteEntry {
    pub name: String,
    pub url: String,
    pub push_url: Option<String>,
}

pub fn list_remotes(repo: &Repository) -> Result<Vec<RemoteEntry>, git2::Error> {
    let mut entries = Vec::new();

    for name in repo.remotes()?.iter().flatten() {
        let remote = repo.find_remote(name)?;
        entries.push(RemoteEntry { name: name.to_string(), url: remote.url().unwrap_or_default().to_string(), push_url: remote.pushurl().map(str::to_string) });
    }

    entries.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(entries)
}

#[cfg(test)]
#[path = "../../tests/git/queries/remotes.rs"]
mod tests;
