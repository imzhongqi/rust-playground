pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut a = a;
    for arr in a.iter_mut() {
        let (mut left, mut right) = (0, arr.len() - 1);
        while left < right {
            if arr[left] == arr[right] {
                arr[right] ^= 1;
                arr[left] ^= 1;
            }
            left += 1;
            right -= 1;
        }
        if left == right {
            arr[left] ^= 1;
        }
    }
    a
}

#[test]
fn test_flip_and_invert_image() {
    assert_eq!(
        flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]),
        vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]
    );
    assert_eq!(
        flip_and_invert_image(vec![
            vec![1, 1, 0, 0],
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0]
        ]),
        vec![
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 1],
            vec![1, 0, 1, 0]
        ]
    );

    assert_eq!(flip_and_invert_image(vec![vec![1],]), vec![vec![0]]);
}
