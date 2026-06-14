use super::*;
use crate::helpers::palette::Theme;
use git2::{Oid, Time};

#[test]
fn duplicate_events_remain_list_rows_but_latest_is_newest() {
    let mut oids = Oids::default();
    let oid = Oid::from_str("1111111111111111111111111111111111111111").unwrap();
    let alias = oids.get_alias_by_oid(oid);
    oids.append_sorted_alias(alias);

    let entries = vec![
        HeadReflogEntry { selector: "HEAD@{0}".to_string(), old_oid: Oid::zero(), new_oid: oid, message: "newest".to_string(), time: Time::new(2, 0) },
        HeadReflogEntry { selector: "HEAD@{1}".to_string(), old_oid: Oid::zero(), new_oid: oid, message: "older".to_string(), time: Time::new(1, 0) },
    ];
    let color = Rc::new(RefCell::new(ColorPicker::from_theme(&Theme::default())));
    let mut lanes = HashMap::new();
    lanes.insert(alias, 0);

    let mut reflogs = HeadReflogs::default();
    reflogs.feed(&oids, &color, &lanes, entries);

    assert_eq!(reflogs.entries.len(), 2);
    assert_eq!(reflogs.latest_for_alias(alias).unwrap().message, "newest");
    assert_eq!(reflogs.indices, vec![1, 1]);
    assert!(reflogs.get_color(alias).is_some());
}
