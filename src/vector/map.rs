/// Create a `HashMap` with the given key-value pairs.
/// There are three ways to use this macro:
/// 1. `map!()`: Create an empty `HashMap`.
/// 2. `map!(usize; 1, 2)`: Create a `HashMap` with the keys `1` and `2` with the default value of `usize`.
/// 3. `map!("one" => 1, "two" => 2)`: Create a `HashMap` with the keys `"one"` and `"two"` with the values `1` and `2` respectively.
/// # Examples
/// ```
/// use std::collections::HashMap;
///
/// let empty_map: HashMap<usize, usize> = lib::map!();
/// assert_eq!(empty_map.len(), 0);
///
/// let map: HashMap<&str, usize> = lib::map!("one" => 1, "two" => 2);
/// assert_eq!(map.len(), 2);
/// assert_eq!(map.get("one"), Some(&1));
/// assert_eq!(map.get("two"), Some(&2));
///
/// let map: HashMap<usize, usize> = lib::map!(usize; 1, 2);
/// assert_eq!(map.len(), 2);
/// assert_eq!(map.get(&1), Some(&0));
/// assert_eq!(map.get(&2), Some(&0));
/// ```
#[macro_export]
macro_rules! map {
    () => { std::collections::HashMap::new() };
    ($default:ty; $($key:expr),* $(,)?) => {
        {
            #[allow(unused_mut)]
            let mut temp_map = std::collections::HashMap::new();
            $(
                temp_map.insert($key, <$default>::default());
            )*
            temp_map
        }
    };
    ($($k:expr => $v:expr),* $(,)?) => {
        {
            let mut temp_map = std::collections::HashMap::new();
            $(
                temp_map.insert($k, $v);
            )*
            temp_map
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_empty_map() {
        let map: HashMap<usize, usize> = map!();
        assert_eq!(map.len(), 0);
    }
    #[test]
    fn test_map() {
        let map = map! {
            "one" => 1,
            "two" => 2,
            "three" => 3,
        };
        assert_eq!(map.len(), 3);
        assert_eq!(map.get("one"), Some(&1));
        assert_eq!(map.get("two"), Some(&2));
        assert_eq!(map.get("three"), Some(&3));
    }

    #[test]
    fn test_map_only_keys() {
        let map: HashMap<usize, usize> = map!(usize; 1, 2, 3);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&1), Some(&0));
        assert_eq!(map.get(&2), Some(&0));
        assert_eq!(map.get(&3), Some(&0));
    }

    #[test]
    fn test_map_only_keys_0_keys() {
        let map: HashMap<usize, usize> = map!(usize;);
        assert_eq!(map.len(), 0);
    }
}
