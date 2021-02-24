use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: HashMap<i32, i32> = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        if let Some(n) = m.get(&(target - v)) {
            return vec![*n, i as i32];
        }
        m.insert(*v, i as i32);
    }
    vec![]
}

#[test]
fn test_two_sum() {
    assert_eq!(two_sum(vec![1, 2, 3], 5), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}
