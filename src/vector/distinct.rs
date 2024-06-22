#[cfg(feature = "vec")]
pub trait Distinct {
    fn distinct(&mut self);
}

#[cfg(feature = "vec")]
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

#[cfg(all(test, feature = "vec"))]
mod tests {
    use super::*;

    #[test]
    fn test_distinct() {
        let mut vec = vec![1, 2, 3, 1, 2, 3];
        vec.distinct();
        assert_eq!(vec, vec![1, 2, 3]);
    }
}
