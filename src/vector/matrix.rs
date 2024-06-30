#[macro_export]
macro_rules! matrix {
    ($x:expr; $m:expr, $n:expr) => {
        vec![vec![$x; $n]; $m]
    };
    ($($($x:expr),*);*) => {
        {
            let mut temp_vec = vec![];
            {} // Needed to avoid clippy warning
            $(
                temp_vec.push(vec![$($x),*]);
            )*
            temp_vec
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_matrix() {
        let matrix = matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
        assert_eq!(matrix, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    }

    #[test]
    fn test_matrix_with_single_value() {
        let matrix = matrix![0; 2, 3];
        assert_eq!(matrix, vec![vec![0, 0, 0], vec![0, 0, 0]]);
    }
}
