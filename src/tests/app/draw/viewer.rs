use super::*;
use crate::git::queries::helpers::FileChanges;

#[test]
fn selected_status_file_helpers_match_rendered_group_order() {
    let mut app = App::default();
    app.uncommitted.conflicts = vec!["conflict.txt".to_string()];
    app.uncommitted.staged = FileChanges { modified: vec!["staged-modified.txt".to_string()], added: vec!["staged-added.txt".to_string()], deleted: vec!["staged-deleted.txt".to_string()] };
    app.uncommitted.unstaged = FileChanges { modified: vec!["unstaged-modified.txt".to_string()], added: vec!["unstaged-added.txt".to_string()], deleted: vec!["unstaged-deleted.txt".to_string()] };

    app.status_top_selected = 0;
    assert!(app.selected_staged_status_file_is_conflict());
    assert_eq!(app.selected_staged_status_file_name().as_deref(), Some("conflict.txt"));

    app.status_top_selected = 2;
    assert!(!app.selected_staged_status_file_is_conflict());
    assert_eq!(app.selected_staged_status_file_name().as_deref(), Some("staged-added.txt"));

    app.status_bottom_selected = 0;
    assert!(app.selected_unstaged_status_file_is_conflict());
    assert_eq!(app.selected_unstaged_status_file_name().as_deref(), Some("conflict.txt"));

    app.status_bottom_selected = 3;
    assert!(!app.selected_unstaged_status_file_is_conflict());
    assert_eq!(app.selected_unstaged_status_file_name().as_deref(), Some("unstaged-deleted.txt"));
}
