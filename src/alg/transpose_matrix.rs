pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if matrix.len() == 0 {
        return vec![];
    }
    let mut result = vec![];
    for i in 0..matrix[0].len() {
        let mut sub_matrix = vec![];
        for j in 0..matrix.len() {
            sub_matrix.push(matrix[j][i]);
        }
        result.push(sub_matrix);
    }
    result
}

#[test]
fn test_transpose() {
    assert_eq!(
        transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
    );
    assert_eq!(
        transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
        vec![vec![1, 4], vec![2, 5], vec![3, 6]]
    );
    assert_eq!(transpose(vec![]), Vec::<Vec<_>>::new());
}
