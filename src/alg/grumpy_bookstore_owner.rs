pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
    let mut max_x: i32 = (0..x as usize)
        .map(|i| if grumpy[i] == 1 { customers[i] } else { 0 })
        .sum();
    let mut max = max_x;

    (x as usize..customers.len()).for_each(|i| {
        if grumpy[i] == 1 {
            max_x += customers[i];
        }
        if grumpy[i - x as usize] == 1 {
            max_x -= customers[i - x as usize]
        }
        if max_x > max {
            max = max_x;
        }
    });

    (0..customers.len())
        .map(|i| if grumpy[i] == 0 { customers[i] } else { 0 })
        .sum::<i32>()
        + max
}

pub fn max_satisfied1(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
    use std::cmp::max;

    let len = customers.len();
    let mut y = vec![0; len];
    let mut z = vec![0; len];
    y[0] = customers[0] & (grumpy[0] - 1);
    z[0] = customers[0];

    let mut sum = customers[0];
    for i in 1..len {
        let v = customers[i] & (grumpy[i] - 1);
        y[i] = v + y[i - 1];

        if i < x as usize {
            sum += customers[i];
            z[i] = max(y[i - 1] + v, sum);
        } else {
            sum += customers[i] - customers[i - x as usize];
            z[i] = max(z[i - 1] + v, y[i - x as usize] + sum);
        }
    }

    max(y[len - 1], z[len - 1])
}

pub fn max_satisfied2(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
    let mut customers = customers;
    let mut sum = 0;
    for i in 0..customers.len() {
        if grumpy[i] == 0 {
            sum += customers[i];
            customers[i] = 0;
        }
    }
    let mut sum_x = 0;
    for i in 0..x as usize {
        sum_x += customers[i];
    }
    let mut max = sum_x;
    for i in x as usize..customers.len() {
        sum_x += customers[i] - customers[i - x as usize];
        if sum_x > max {
            max = sum_x;
        }
    }
    sum + max
}

#[test]
fn test_max_satisfied() {
    assert_eq!(
        max_satisfied(
            vec![1, 0, 1, 2, 1, 1, 7, 5],
            vec![0, 1, 0, 1, 0, 1, 0, 1],
            3
        ),
        16
    );
    assert_eq!(max_satisfied(vec![8, 8], vec![1, 0], 2), 16);
    assert_eq!(max_satisfied(vec![4, 10, 10], vec![1, 1, 0], 2), 24);
    assert_eq!(max_satisfied(vec![2, 6, 6, 9], vec![0, 0, 1, 1], 1), 17);
}
