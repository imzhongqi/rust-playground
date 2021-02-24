pub fn roman_to_int(s: String) -> i32 {
    let mut n = 0;
    let mut idx: usize = 0;
    let sb = s.as_bytes();
    while idx < s.len() {
        let cn = match sb[idx] {
            b'I' => match sb.get(idx + 1) {
                Some(b'V') | Some(b'X') => -1,
                _ => 1,
            },
            b'V' => 5,
            b'X' => match sb.get(idx + 1) {
                Some(b'L') | Some(b'C') => -10,
                _ => 10,
            },
            b'L' => 50,
            b'C' => match sb.get(idx + 1) {
                Some(b'D') | Some(b'M') => -100,
                _ => 100,
            },
            b'D' => 500,
            b'M' => 1000,
            _ => 0,
        };
        n += cn;
        idx += 1;
    }
    n
}

#[test]
pub fn test_roman_to_int() {
    assert_eq!(roman_to_int(String::from("III")), 3);
    assert_eq!(roman_to_int(String::from("IV")), 4);
    assert_eq!(roman_to_int(String::from("IX")), 9);
    assert_eq!(roman_to_int(String::from("LVIII")), 58);
    assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
}
