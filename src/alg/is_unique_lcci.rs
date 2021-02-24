pub fn is_unique(astr: String) -> bool {
    let mut bit_map = 0;
    for c in astr.as_bytes() {
        let key = c - 97;
        if bit_map & (1 << key) != 0 {
            return false;
        }
        bit_map |= 1 << key;
    }
    true
}

pub fn is_unique1(astr: String) -> bool {
    for c in astr.as_bytes() {
        let mut s = astr.clone();
        s = s.replace(&String::from(*c as char), "");
        if s.len() != astr.len() - 1 {
            return false;
        }
    }
    true
}

#[test]
fn test_is_unique() {
    assert_eq!(is_unique(String::from("leetcode")), false);
    assert_eq!(is_unique(String::from("abc")), true);
    assert_eq!(is_unique(String::from("aa")), false);
    assert_eq!(is_unique(String::from("")), true);
}
