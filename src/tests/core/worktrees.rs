use super::*;

fn linked_entry(current: bool, locked: Option<&str>) -> WorktreeEntry {
    WorktreeEntry {
        name: "feature".into(),
        path: PathBuf::from("/tmp/feature"),
        branch: Some("feature".into()),
        head: None,
        alias: None,
        kind: WorktreeKind::Linked,
        is_current: current,
        is_valid: true,
        is_prunable: false,
        locked_reason: locked.map(str::to_string),
        is_dirty: false,
    }
}

#[test]
fn guards_current_main_and_locked_removal() {
    let mut main = linked_entry(false, None);
    main.kind = WorktreeKind::Main;
    assert!(!main.can_remove());
    assert!(!main.can_lock());

    let current = linked_entry(true, None);
    assert!(!current.can_remove());
    assert!(current.can_lock());

    let locked = linked_entry(false, Some("keep"));
    assert!(!locked.can_remove());
    assert!(locked.can_lock());

    let removable = linked_entry(false, None);
    assert!(removable.can_remove());
}
