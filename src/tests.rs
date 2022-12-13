#[cfg(test)]
mod tests {

    #[test]
    fn test_determinant_cramer() {}

    #[test]
    fn test_determinant_bezout() {}

    #[test]
    fn test_extracte_submatrix() {}

    #[test]
    fn test_sign() {
        assert_eq!(1, crate::sign(vec![0, 1, 2]));
        assert_eq!(-1, crate::sign(vec![1, 0, 2]));
    }

    #[test]
    fn test_generate_permutations() {
        assert_eq!(crate::generate_permuations(2), vec![vec![0, 1], vec![1, 0]]);
        assert_eq!(
            crate::generate_permuations(3).sort(),
            vec![
                vec![0, 1, 2],
                vec![1, 0, 2],
                vec![2, 1, 0],
                vec![0, 2, 1],
                vec![1, 2, 0],
                vec![2, 0, 1],
            ]
            .sort()
        );
    }
}
