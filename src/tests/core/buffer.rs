use super::*;
use crate::core::chunk::{Chunk, NONE};

#[test]
fn decompress_rebuilds_visible_range_from_delta_history() {
    let mut buffer = Buffer::default();

    buffer.update(Chunk::commit(2, 1, NONE));
    buffer.update(Chunk::commit(1, NONE, NONE));
    buffer.backup();

    buffer.decompress(1, buffer.deltas.len());

    assert_eq!(buffer.history.len(), 2);
    assert_eq!(buffer.history[0].len(), 1);
    assert_eq!(buffer.history[0][0].alias, 2);
    assert_eq!(buffer.history[0][0].parent_a, 1);
    assert_eq!(buffer.history[1].len(), 1);
    assert_eq!(buffer.history[1][0].alias, 1);
    assert_eq!(buffer.history[1][0].parent_a, NONE);
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
