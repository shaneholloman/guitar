use crate::git::{
    actions::{
        fetching::fetch_remote,
        pushing::{delete_remote_branch, push_branch, push_tags},
        submodules::update_submodule,
    },
    auth::{AuthSession, NetworkResult},
};
use std::thread;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRequest {
    Fetch { repo_path: String, remote_name: String },
    PushBranch { repo_path: String, remote_name: String, branch: String, force: bool },
    PushTags { repo_path: String, remote_name: String },
    DeleteRemoteBranch { repo_path: String, remote_name: String, branch: String },
    UpdateSubmodule { repo_path: String, name: String },
}

impl NetworkRequest {
    pub fn label(&self) -> &'static str {
        match self {
            NetworkRequest::Fetch { .. } => "Fetch",
            NetworkRequest::PushBranch { .. } => "Push",
            NetworkRequest::PushTags { .. } => "Push tags",
            NetworkRequest::DeleteRemoteBranch { .. } => "Delete remote branch",
            NetworkRequest::UpdateSubmodule { .. } => "Update submodule",
        }
    }

    pub fn progress_message(&self) -> String {
        match self {
            NetworkRequest::Fetch { remote_name, .. } => format!("Fetching {remote_name}..."),
            NetworkRequest::PushBranch { remote_name, branch, force, .. } => {
                if *force {
                    format!("Force pushing {branch} to {remote_name}...")
                } else {
                    format!("Pushing {branch} to {remote_name}...")
                }
            },
            NetworkRequest::PushTags { remote_name, .. } => format!("Pushing local tags to {remote_name}..."),
            NetworkRequest::DeleteRemoteBranch { remote_name, branch, .. } => format!("Deleting {remote_name}/{branch}..."),
            NetworkRequest::UpdateSubmodule { name, .. } => format!("Updating submodule {name}..."),
        }
    }

    pub fn spawn(&self, auth_session: AuthSession) -> thread::JoinHandle<NetworkResult> {
        match self {
            NetworkRequest::Fetch { repo_path, remote_name } => fetch_remote(repo_path, remote_name, auth_session),
            NetworkRequest::PushBranch { repo_path, remote_name, branch, force } => push_branch(repo_path, remote_name, branch, *force, auth_session),
            NetworkRequest::PushTags { repo_path, remote_name } => push_tags(repo_path, remote_name, auth_session),
            NetworkRequest::DeleteRemoteBranch { repo_path, remote_name, branch } => delete_remote_branch(repo_path, remote_name, branch, auth_session),
            NetworkRequest::UpdateSubmodule { repo_path, name } => update_submodule(repo_path, name, auth_session),
        }
    }
}
