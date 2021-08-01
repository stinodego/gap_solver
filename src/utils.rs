use std::collections::HashMap;

/// Find the max key in a map with float values
pub fn max_key_by_value<T>(map: &HashMap<T, f64>) -> Option<&T> {
    let mut map_iter = map.iter();
    let (mut max_key, mut max_value) = match map_iter.next() {
        Some(x) => x,
        _ => return None,
    };

    for (key, value) in map_iter {
        if value > max_value {
            max_key = key;
            max_value = value;
        }
    }
    Some(max_key)
}

#[test]
fn test_max_key_by_value() {
    let mut input = HashMap::new();
    input.insert("a", 1.0);
    input.insert("b", 2.0);
    let result = max_key_by_value(&input);
    assert_eq!(result, Some(&"b"));
}
