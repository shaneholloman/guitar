use crate::git::{
    actions::conflicts::{ensure_clean_workdir, mark_conflicts_resolved_from_workdir},
    queries::commits::get_current_branch,
};
use git2::{Error, Oid, Rebase, RebaseOptions, Repository, RepositoryState, build::CheckoutBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RebaseOutcome {
    Completed { applied: usize },
    Conflict,
    Aborted,
}

fn is_rebase_state(state: RepositoryState) -> bool {
    matches!(state, RepositoryState::Rebase | RepositoryState::RebaseInteractive | RepositoryState::RebaseMerge | RepositoryState::ApplyMailboxOrRebase)
}

pub fn is_rebase_in_progress(repo: &Repository) -> bool {
    is_rebase_state(repo.state())
}

fn rebase_options<'a>() -> RebaseOptions<'a> {
    let mut checkout = CheckoutBuilder::new();
    checkout.allow_conflicts(true).conflict_style_merge(true);

    let mut opts = RebaseOptions::new();
    opts.checkout_options(checkout);
    opts
}

fn drive_rebase(repo: &Repository, rebase: &mut Rebase<'_>, mut applied: usize) -> Result<RebaseOutcome, Error> {
    let signature = repo.signature()?;

    loop {
        match rebase.next() {
            Some(Ok(_)) => {
                if repo.index()?.has_conflicts() {
                    return Ok(RebaseOutcome::Conflict);
                }
                rebase.commit(None, &signature, None)?;
                applied += 1;
            },
            Some(Err(error)) => return Err(error),
            None => {
                rebase.finish(Some(&signature))?;
                return Ok(RebaseOutcome::Completed { applied });
            },
        }
    }
}

pub fn start_rebase(repo: &Repository, upstream_oid: Oid) -> Result<RebaseOutcome, Error> {
    if get_current_branch(repo).is_none() {
        return Err(Error::from_str("rebasing requires a checked-out local branch"));
    }
    if is_rebase_in_progress(repo) {
        return Err(Error::from_str("rebase already in progress"));
    }

    let head_oid = repo.head()?.target().ok_or_else(|| Error::from_str("HEAD does not point to a commit"))?;
    if head_oid == upstream_oid {
        return Err(Error::from_str("selected commit is already HEAD"));
    }

    ensure_clean_workdir(repo, "rebasing")?;

    let upstream = repo.find_annotated_commit(upstream_oid)?;
    let mut opts = rebase_options();
    let mut rebase = repo.rebase(None, Some(&upstream), None, Some(&mut opts))?;
    drive_rebase(repo, &mut rebase, 0)
}

pub fn continue_rebase(repo: &Repository) -> Result<RebaseOutcome, Error> {
    if !is_rebase_in_progress(repo) {
        return Err(Error::from_str("no rebase in progress"));
    }

    mark_conflicts_resolved_from_workdir(repo)?;
    if repo.index()?.has_conflicts() {
        return Ok(RebaseOutcome::Conflict);
    }

    let mut opts = rebase_options();
    let mut rebase = repo.open_rebase(Some(&mut opts))?;
    let signature = repo.signature()?;
    let mut applied = 0;

    if rebase.operation_current().is_some() {
        rebase.commit(None, &signature, None)?;
        applied += 1;
    }

    drive_rebase(repo, &mut rebase, applied)
}

pub fn abort_rebase(repo: &Repository) -> Result<RebaseOutcome, Error> {
    if !is_rebase_in_progress(repo) {
        return Err(Error::from_str("no rebase in progress"));
    }

    let mut opts = rebase_options();
    let mut rebase = repo.open_rebase(Some(&mut opts))?;
    rebase.abort()?;
    Ok(RebaseOutcome::Aborted)
}

#[cfg(test)]
#[path = "../../tests/git/actions/rebasing.rs"]
mod tests;
