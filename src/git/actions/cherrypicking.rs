use crate::git::actions::conflicts::{ensure_clean_workdir, mark_conflicts_resolved_from_workdir};
use git2::{CherrypickOptions, Error, Oid, Repository, RepositoryState, build::CheckoutBuilder};
use std::{fs, path::PathBuf};

const GUITAR_CHERRYPICK_MSG: &str = "GUITAR_CHERRYPICK_MSG";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CherrypickOutcome {
    Committed { oid: Oid },
    Conflict,
    Aborted,
}

pub fn is_cherrypick_in_progress(repo: &Repository) -> bool {
    matches!(repo.state(), RepositoryState::CherryPick | RepositoryState::CherryPickSequence)
}

fn message_path(repo: &Repository) -> PathBuf {
    repo.path().join(GUITAR_CHERRYPICK_MSG)
}

fn persist_message(repo: &Repository, message: &str) -> Result<(), Error> {
    fs::write(message_path(repo), message).map_err(|error| Error::from_str(&format!("write cherry-pick message failed: {error}")))
}

fn cleanup_message(repo: &Repository) {
    let _ = fs::remove_file(message_path(repo));
}

fn read_message(repo: &Repository) -> String {
    fs::read_to_string(message_path(repo))
        .ok()
        .filter(|message| !message.trim().is_empty())
        .or_else(|| fs::read_to_string(repo.path().join("MERGE_MSG")).ok())
        .filter(|message| !message.trim().is_empty())
        .unwrap_or_else(|| "cherrypicked: Cherry-pick commit".to_string())
}

fn cherrypick_options<'a>() -> CherrypickOptions<'a> {
    let mut checkout = CheckoutBuilder::new();
    checkout.allow_conflicts(true).conflict_style_merge(true);

    let mut opts = CherrypickOptions::new();
    opts.checkout_builder(checkout);
    opts
}

fn commit_index(repo: &Repository, message: &str) -> Result<Oid, Error> {
    let mut index = repo.index()?;
    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;
    let sig = repo.signature()?;
    let head_commit = repo.head()?.peel_to_commit()?;
    let parents = [&head_commit];
    let oid = repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)?;
    repo.cleanup_state()?;
    cleanup_message(repo);
    repo.checkout_head(Some(CheckoutBuilder::default().force()))?;
    Ok(oid)
}

pub fn start_cherrypick(repo: &Repository, commit_oid: Oid, message: &str) -> Result<CherrypickOutcome, Error> {
    if is_cherrypick_in_progress(repo) {
        return Err(Error::from_str("cherry-pick already in progress"));
    }
    ensure_clean_workdir(repo, "cherry-picking")?;
    persist_message(repo, message)?;

    let commit = repo.find_commit(commit_oid)?;
    let mut opts = cherrypick_options();
    if let Err(error) = repo.cherrypick(&commit, Some(&mut opts)) {
        cleanup_message(repo);
        return Err(error);
    }

    if repo.index()?.has_conflicts() {
        return Ok(CherrypickOutcome::Conflict);
    }

    commit_index(repo, message).map(|oid| CherrypickOutcome::Committed { oid })
}

pub fn continue_cherrypick(repo: &Repository) -> Result<CherrypickOutcome, Error> {
    if !is_cherrypick_in_progress(repo) {
        return Err(Error::from_str("no cherry-pick in progress"));
    }

    mark_conflicts_resolved_from_workdir(repo)?;
    if repo.index()?.has_conflicts() {
        return Ok(CherrypickOutcome::Conflict);
    }

    let message = read_message(repo);
    commit_index(repo, &message).map(|oid| CherrypickOutcome::Committed { oid })
}

pub fn abort_cherrypick(repo: &Repository) -> Result<CherrypickOutcome, Error> {
    if !is_cherrypick_in_progress(repo) {
        return Err(Error::from_str("no cherry-pick in progress"));
    }

    repo.reset(&repo.head()?.peel_to_commit()?.into_object(), git2::ResetType::Hard, Some(CheckoutBuilder::default().force()))?;
    repo.cleanup_state()?;
    cleanup_message(repo);
    Ok(CherrypickOutcome::Aborted)
}

#[cfg(test)]
#[path = "../../tests/git/actions/cherrypicking.rs"]
mod tests;
