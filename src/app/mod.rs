#[allow(clippy::module_inception)]
pub mod app;

pub mod draw {
    pub mod branches;
    pub mod graph;
    pub mod inspector;
    pub mod modals {
        pub mod checkout;
        pub mod delete_branch;
        pub mod delete_tag;
        pub mod error;
        pub mod input;
        pub mod rebase;
        pub mod remove_worktree;
        pub mod solo;
        pub mod worktree_chooser;
    }
    pub mod reflogs;
    pub mod settings;
    pub mod splash;
    pub mod stashes;
    pub mod status;
    pub mod statusbar;
    pub mod tags;
    pub mod title;
    pub mod viewer;
    pub mod worktrees;
}

pub mod input {
    pub mod events;
    pub mod git;
    pub mod handler;
    pub mod modals;
    pub mod navigation;
    pub mod text;
    pub mod worktrees;

    pub use text::TextInput;
}

pub mod state {
    pub mod defaults;
    pub mod layout;
}
