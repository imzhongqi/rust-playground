pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    for i in 0..matrix.len() - 1 {
        for j in 0..matrix[i].len() - 1 {
            if matrix[i][j] != matrix[i + 1][j + 1] {
                return false;
            }
        }
    }
    true
}

#[test]
fn test_is_toeplitz_matrix() {
    assert_eq!(
        is_toeplitz_matrix(vec! {
            vec![1, 2],
            vec![2, 2]
        }),
        false
    );

    assert_eq!(
        is_toeplitz_matrix(vec! {
            vec![1, 2, 3, 4],
            vec![5, 1, 2, 3],
            vec![9, 5, 1, 2]
        }),
        true,
    );
}
