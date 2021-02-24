pub fn reverse(x: i32) -> i32 {
    let (mut num, mut reverse_num): (i32, i32) = (x, 0);
    while num != 0 {
        let digit = num % 10;
        num /= 10;
        if let Some(rev_num) = reverse_num.checked_mul(10) {
            reverse_num = rev_num + digit;
        } else {
            return 0;
        }
    }
    reverse_num
}

#[test]
fn test_reverse() {
    assert_eq!(reverse(100), 1);
    assert_eq!(reverse(-321), -123);
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(1534236469), 0);
}
