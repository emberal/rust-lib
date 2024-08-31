/// Modify self to contain only distinct elements.
pub trait Distinct {
    /// Modify self to contain only distinct elements.
    fn distinct(&mut self);
}

impl<T: PartialEq + Clone> Distinct for Vec<T> {
    fn distinct(&mut self) {
        *self = self.iter().fold(vec![], |mut acc, x| {
            if !acc.contains(x) {
                acc.push(x.clone());
            }
            acc
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct() {
        let mut vec = vec![1, 2, 3, 1, 2, 3];
        vec.distinct();
        assert_eq!(vec, vec![1, 2, 3]);
    }
}
