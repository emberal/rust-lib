#[macro_export]
macro_rules! map {
    () => { std::collections::HashMap::new() };
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
}
