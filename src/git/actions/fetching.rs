use git2::FetchPrune;
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use std::thread;

// Run fetch on a worker thread so the TUI can decide when to block or redraw.
pub fn fetch_over_ssh(repo_path: &str, remote_name: &str) -> thread::JoinHandle<Result<(), git2::Error>> {
    // Own the inputs before crossing the thread boundary.
    let repo_path = repo_path.to_string();
    let remote_name = remote_name.to_string();

    thread::spawn(move || {
        let repo = Repository::open(repo_path)?;
        let mut remote = repo.find_remote(&remote_name)?;

        let mut callbacks = RemoteCallbacks::new();
        // Use the same SSH agent flow as git fetch for common git@host remotes.
        callbacks.credentials(|_url, username_from_url, _| Cred::ssh_key_from_agent(username_from_url.unwrap_or("git")));

        callbacks.transfer_progress(|_stats| {
            // println!("Received {}/{} objects", stats.received_objects(), stats.total_objects());
            true
        });

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        fetch_options.prune(FetchPrune::On);

        // Fetch heads and tags explicitly because libgit2 does not expand all refspecs by default.
        remote.fetch(&["refs/heads/*:refs/remotes/origin/*", "refs/tags/*:refs/tags/*"], Some(&mut fetch_options), None)?;
        Ok(())
    })
}
