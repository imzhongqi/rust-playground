pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut left, mut right, mut max_area) = (0, height.len() - 1, 0);
    while left < right {
        let calc_area = |height| max_area.max((right - left) as i32 * height);
        if height[left] < height[right] {
            max_area = calc_area(height[left]);
            left += 1;
        } else {
            max_area = calc_area(height[right]);
            right -= 1;
        }
    }
    max_area
}

#[test]
fn test_max_area() {
    assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(max_area(vec![1, 1]), 1);
    assert_eq!(max_area(vec![4, 3, 2, 1, 4]), 16);
    assert_eq!(max_area(vec![1, 2, 1]), 2);
    assert_eq!(max_area(vec![1, 2]), 1);
}
