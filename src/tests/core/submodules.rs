use super::*;

fn entry(name: &str, open: bool, dirty: bool) -> SubmoduleEntry {
    SubmoduleEntry {
        name: name.into(),
        path: PathBuf::from(name),
        absolute_path: PathBuf::from(format!("/tmp/{name}")),
        url: Some(format!("https://example.com/{name}.git")),
        branch: Some("main".into()),
        head: None,
        index: None,
        workdir: None,
        is_open: open,
        is_uninitialized: !open,
        is_in_head: true,
        is_in_index: true,
        is_in_config: true,
        is_in_workdir: open,
        is_index_modified: dirty,
        is_workdir_modified: false,
        has_new_commits: false,
        has_modified_content: false,
        has_untracked_content: false,
    }
}

#[test]
fn submodule_entry_reports_open_and_dirty_state() {
    let clean = entry("clean", true, false);
    assert!(clean.can_open());
    assert!(!clean.is_dirty());

    let closed = entry("closed", false, false);
    assert!(!closed.can_open());

    let dirty = entry("dirty", true, true);
    assert!(dirty.is_dirty());
}

#[test]
fn submodules_wrap_entries() {
    let submodules = Submodules::from_entries(vec![entry("a", true, false), entry("b", false, false)]);

    assert_eq!(submodules.entries.len(), 2);
    assert_eq!(submodules.entries[0].name, "a");
}

#[test]
fn submodule_stack_entry_stores_parent_and_child_paths() {
    let entry = SubmoduleStackEntry::new(PathBuf::from("/repo"), PathBuf::from("deps/child"), "deps/child".into());

    assert_eq!(entry.parent_path, PathBuf::from("/repo"));
    assert_eq!(entry.submodule_path, PathBuf::from("deps/child"));
    assert_eq!(entry.submodule_name, "deps/child");
}
