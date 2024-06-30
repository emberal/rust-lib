#[macro_export]
macro_rules! set {
    () => { std::collections::HashSet::new() };
    ($($x:expr),* $(,)?) => {
        {
            let mut temp_set = std::collections::HashSet::new();
            $(
                temp_set.insert($x);
            )*
            temp_set
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn test_empty_set() {
        let set: HashSet<usize> = set![];
        assert_eq!(set.len(), 0);
    }
    #[test]
    fn test_set() {
        let set = set![1, 2, 3];
        assert_eq!(set.len(), 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }
}
