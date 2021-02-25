pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, height.len() - 1);
    let mut area = 0;
    while left < right {
        let h = if height[left] < height[right] {
            height[left]
        } else {
            height[right]
        };
        let a = (right - left) as i32 * h;
        area = if a > area { a } else { area };
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    area
}

#[test]
fn test_max_area() {
    assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(max_area(vec![1, 1]), 1);
    assert_eq!(max_area(vec![4, 3, 2, 1, 4]), 16);
    assert_eq!(max_area(vec![1, 2, 1]), 2);
    assert_eq!(max_area(vec![1, 2]), 1);
}
