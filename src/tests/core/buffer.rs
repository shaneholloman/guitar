use super::*;
use crate::core::chunk::{Chunk, NONE};

#[test]
fn window_rebuilds_visible_range_from_delta_history() {
    let mut buffer = Buffer::default();

    buffer.update(Chunk::commit(2, 1, NONE));
    buffer.update(Chunk::commit(1, NONE, NONE));
    buffer.backup();

    let history = buffer.window(1, buffer.deltas.len());

    assert_eq!(history.len(), 2);
    assert_eq!(history[0].len(), 1);
    assert_eq!(history[0][0].alias, 2);
    assert_eq!(history[0][0].parent_a, 1);
    assert_eq!(history[1].len(), 1);
    assert_eq!(history[1][0].alias, 1);
    assert_eq!(history[1][0].parent_a, NONE);
}

#[test]
fn window_does_not_mutate_current_graph_state() {
    let mut buffer = Buffer::default();

    buffer.update(Chunk::commit(3, 2, NONE));
    buffer.update(Chunk::commit(2, 1, NONE));
    buffer.update(Chunk::commit(1, NONE, NONE));
    buffer.backup();

    let before = buffer.curr.clone();
    let window = buffer.window(1, buffer.deltas.len());

    assert_eq!(buffer.curr, before);
    assert_eq!(window.len(), 3);
    assert_eq!(window[0][0].alias, 3);
    assert_eq!(window[1][0].alias, 2);
    assert_eq!(window[2][0].alias, 1);
}

#[test]
fn planned_merger_splits_lane_before_replacing_first_parent() {
    let mut buffer = Buffer::default();

    buffer.update(Chunk::commit(3, 1, 2));
    buffer.merger(3);
    buffer.update(Chunk::commit(1, NONE, NONE));

    assert!(buffer.mergers.is_empty());
    assert_eq!(buffer.curr.len(), 2);
    assert_eq!(buffer.curr[0].alias, 1);
    assert_eq!(buffer.curr[0].parent_a, NONE);
    assert_eq!(buffer.curr[1].alias, 3);
    assert_eq!(buffer.curr[1].parent_a, 2);
    assert_eq!(buffer.curr[1].parent_b, NONE);
}
