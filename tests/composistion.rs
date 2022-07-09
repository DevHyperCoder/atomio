use atomio::{parser::Parsed,unit::Unit};
use std::collections::HashMap;

#[test]
fn test_composition() {
    let parsed: Parsed = "CH4".try_into().unwrap();

    let comp_map = parsed.root_unit.get_composition();

    assert_eq!( comp_map.get(&"C".to_string()), Some(&1));
    assert_eq!( comp_map.get(&"H".to_string()), Some(&4));
}
