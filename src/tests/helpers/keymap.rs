use super::*;

#[test]
fn migration_adds_worktree_defaults_without_overwriting_existing_keys() {
    let mut maps = IndexMap::new();
    let mut normal = IndexMap::new();
    normal.insert(KeyBinding::new(Char('7'), KeyModifiers::NONE), Command::ToggleTags);
    let mut action = IndexMap::new();
    action.insert(KeyBinding::new(Char('x'), KeyModifiers::NONE), Command::Drop);
    maps.insert(InputMode::Normal, normal);
    maps.insert(InputMode::Action, action);

    assert!(migrate_default_bindings(&mut maps));

    let normal = maps.get(&InputMode::Normal).unwrap();
    let action = maps.get(&InputMode::Action).unwrap();

    assert_eq!(normal.get(&KeyBinding::new(Char('7'), KeyModifiers::NONE)), Some(&Command::ToggleTags));
    assert_eq!(normal.get(&KeyBinding::new(Char('0'), KeyModifiers::NONE)), Some(&Command::ResetLayout));
    assert_eq!(normal.get(&KeyBinding::new(Char('6'), KeyModifiers::NONE)), Some(&Command::ToggleWorktrees));
    assert_eq!(normal.get(&KeyBinding::new(Char('8'), KeyModifiers::NONE)), Some(&Command::ToggleShas));
    assert_eq!(normal.get(&KeyBinding::new(Char('9'), KeyModifiers::NONE)), Some(&Command::ToggleGraphReflogs));
    assert_eq!(normal.get(&KeyBinding::new(Char('w'), KeyModifiers::NONE)), Some(&Command::CreateWorktree));
    assert_eq!(action.get(&KeyBinding::new(Char('W'), KeyModifiers::SHIFT)), Some(&Command::RemoveWorktree));
    assert_eq!(action.get(&KeyBinding::new(Char('L'), KeyModifiers::SHIFT)), Some(&Command::ToggleWorktreeLock));
}

#[test]
fn defaults_include_numeric_ui_toggles() {
    let maps = default_keymaps();
    let normal = maps.get(&InputMode::Normal).unwrap();
    let action = maps.get(&InputMode::Action).unwrap();

    for mode_map in [normal, action] {
        assert_eq!(mode_map.get(&KeyBinding::new(Char('0'), KeyModifiers::NONE)), Some(&Command::ResetLayout));
        assert_eq!(mode_map.get(&KeyBinding::new(Char('1'), KeyModifiers::NONE)), Some(&Command::ToggleBranches));
        assert_eq!(mode_map.get(&KeyBinding::new(Char('2'), KeyModifiers::NONE)), Some(&Command::ToggleTags));
        assert_eq!(mode_map.get(&KeyBinding::new(Char('3'), KeyModifiers::NONE)), Some(&Command::ToggleStashes));
        assert_eq!(mode_map.get(&KeyBinding::new(Char('4'), KeyModifiers::NONE)), Some(&Command::ToggleStatus));
        assert_eq!(mode_map.get(&KeyBinding::new(Char('5'), KeyModifiers::NONE)), Some(&Command::ToggleInspector));
        assert_eq!(mode_map.get(&KeyBinding::new(Char('6'), KeyModifiers::NONE)), Some(&Command::ToggleWorktrees));
        assert_eq!(mode_map.get(&KeyBinding::new(Char('7'), KeyModifiers::NONE)), Some(&Command::ToggleReflogs));
        assert_eq!(mode_map.get(&KeyBinding::new(Char('8'), KeyModifiers::NONE)), Some(&Command::ToggleShas));
        assert_eq!(mode_map.get(&KeyBinding::new(Char('9'), KeyModifiers::NONE)), Some(&Command::ToggleGraphReflogs));
    }
}

#[test]
fn migration_remaps_old_numeric_ui_defaults() {
    let mut maps = IndexMap::new();

    for mode in [InputMode::Normal, InputMode::Action] {
        let mut mode_map = IndexMap::new();
        mode_map.insert(KeyBinding::new(Char('6'), KeyModifiers::NONE), Command::ToggleShas);
        mode_map.insert(KeyBinding::new(Char('7'), KeyModifiers::NONE), Command::ToggleWorktrees);
        mode_map.insert(KeyBinding::new(Char('8'), KeyModifiers::NONE), Command::ToggleReflogs);
        maps.insert(mode, mode_map);
    }

    assert!(migrate_default_bindings(&mut maps));

    for mode in [InputMode::Normal, InputMode::Action] {
        let mode_map = maps.get(&mode).unwrap();
        assert_eq!(mode_map.get(&KeyBinding::new(Char('6'), KeyModifiers::NONE)), Some(&Command::ToggleWorktrees));
        assert_eq!(mode_map.get(&KeyBinding::new(Char('7'), KeyModifiers::NONE)), Some(&Command::ToggleReflogs));
        assert_eq!(mode_map.get(&KeyBinding::new(Char('8'), KeyModifiers::NONE)), Some(&Command::ToggleShas));
    }
}

#[test]
fn defaults_include_operation_bindings() {
    let maps = default_keymaps();
    let action = maps.get(&InputMode::Action).unwrap();

    assert_eq!(action.get(&KeyBinding::new(Char('r'), KeyModifiers::NONE)), Some(&Command::Rebase));
    assert_eq!(action.get(&KeyBinding::new(Char('m'), KeyModifiers::NONE)), Some(&Command::Merge));
    assert_eq!(action.get(&KeyBinding::new(Char('C'), KeyModifiers::SHIFT)), Some(&Command::ContinueOperation));
    assert_eq!(action.get(&KeyBinding::new(Char('A'), KeyModifiers::SHIFT)), Some(&Command::AbortOperation));
}

#[test]
fn migration_adds_operation_defaults_without_rewriting_existing_keys() {
    let mut maps = IndexMap::new();
    maps.insert(InputMode::Normal, IndexMap::new());
    let mut action = IndexMap::new();
    action.insert(KeyBinding::new(Char('r'), KeyModifiers::NONE), Command::Reload);
    action.insert(KeyBinding::new(Char('R'), KeyModifiers::SHIFT), Command::ForcePush);
    maps.insert(InputMode::Action, action);

    assert!(migrate_default_bindings(&mut maps));

    let action = maps.get(&InputMode::Action).unwrap();
    assert_eq!(action.get(&KeyBinding::new(Char('r'), KeyModifiers::NONE)), Some(&Command::Reload));
    assert_eq!(action.get(&KeyBinding::new(Char('R'), KeyModifiers::SHIFT)), Some(&Command::ForcePush));
    assert_eq!(action.get(&KeyBinding::new(Char('m'), KeyModifiers::NONE)), Some(&Command::Merge));
    assert_eq!(action.get(&KeyBinding::new(Char('C'), KeyModifiers::SHIFT)), Some(&Command::ContinueOperation));
    assert_eq!(action.get(&KeyBinding::new(Char('A'), KeyModifiers::SHIFT)), Some(&Command::AbortOperation));
}

#[test]
fn migration_replaces_inherited_action_hunk_mode_with_merge() {
    let mut maps = IndexMap::new();
    maps.insert(InputMode::Normal, IndexMap::new());
    let mut action = IndexMap::new();
    action.insert(KeyBinding::new(Char('m'), KeyModifiers::NONE), Command::ToggleHunkMode);
    maps.insert(InputMode::Action, action);

    assert!(migrate_default_bindings(&mut maps));

    let action = maps.get(&InputMode::Action).unwrap();
    assert_eq!(action.get(&KeyBinding::new(Char('m'), KeyModifiers::NONE)), Some(&Command::Merge));
}

#[test]
fn migration_preserves_custom_action_m_binding() {
    let mut maps = IndexMap::new();
    maps.insert(InputMode::Normal, IndexMap::new());
    let mut action = IndexMap::new();
    action.insert(KeyBinding::new(Char('m'), KeyModifiers::NONE), Command::MixedReset);
    maps.insert(InputMode::Action, action);

    assert!(migrate_default_bindings(&mut maps));

    let action = maps.get(&InputMode::Action).unwrap();
    assert_eq!(action.get(&KeyBinding::new(Char('m'), KeyModifiers::NONE)), Some(&Command::MixedReset));
}

#[test]
fn action_settings_filter_hides_only_identical_inherited_bindings() {
    let mut normal = IndexMap::new();
    normal.insert(KeyBinding::new(Char('j'), KeyModifiers::NONE), Command::ScrollDown);
    normal.insert(KeyBinding::new(Char('r'), KeyModifiers::NONE), Command::Reload);

    let mut action = IndexMap::new();
    action.insert(KeyBinding::new(Char('j'), KeyModifiers::NONE), Command::ScrollDown);
    action.insert(KeyBinding::new(Char('r'), KeyModifiers::NONE), Command::Rebase);
    action.insert(KeyBinding::new(Char('m'), KeyModifiers::NONE), Command::Merge);
    action.insert(KeyBinding::new(Char('C'), KeyModifiers::SHIFT), Command::ContinueOperation);
    action.insert(KeyBinding::new(Char('A'), KeyModifiers::SHIFT), Command::AbortOperation);

    let visible = action_keymap_visible_entries(Some(&normal), &action);

    assert_eq!(visible.get(&KeyBinding::new(Char('j'), KeyModifiers::NONE)), None);
    assert_eq!(visible.get(&KeyBinding::new(Char('r'), KeyModifiers::NONE)), Some(&Command::Rebase));
    assert_eq!(visible.get(&KeyBinding::new(Char('m'), KeyModifiers::NONE)), Some(&Command::Merge));
    assert_eq!(visible.get(&KeyBinding::new(Char('C'), KeyModifiers::SHIFT)), Some(&Command::ContinueOperation));
    assert_eq!(visible.get(&KeyBinding::new(Char('A'), KeyModifiers::SHIFT)), Some(&Command::AbortOperation));
}
