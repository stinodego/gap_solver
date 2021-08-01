use std::collections::HashMap;

/// Find the max key in a map with float values
pub fn max_key_by_value<K, V>(map: &HashMap<K, V>) -> Option<&K>
where
    V: PartialOrd,
{
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

#[test]
fn test_max_key_by_value_empty() {
    let input: HashMap<u32, f64> = HashMap::new();
    let result = max_key_by_value(&input);
    assert_eq!(result, None);
}

#[test]
fn test_max_key_by_value_multiple_max() {
    let mut input = HashMap::new();
    input.insert("a", 2.0);
    input.insert("b", 2.0);
    input.insert("c", 1.0);
    let result = max_key_by_value(&input);
    assert!(result == Some(&"a") || result == Some(&"b"));
}
