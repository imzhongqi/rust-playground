pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = String::new();
    if strs.len() == 0 {
        return prefix;
    }

    let mut col = 0;
    'outer: loop {
        let mut bb = 0u8;
        for s in strs.iter() {
            if col >= s.len() {
                break 'outer;
            }
            let b = s.as_bytes()[col];
            if bb != 0 && b != bb {
                break 'outer;
            }
            bb = b;
        }
        prefix.push(bb as char);
        col += 1;
    }
    prefix
}

#[test]
fn test_longest_common_prefix() {
    assert_eq!(
        longest_common_prefix(vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight")
        ]),
        String::from("fl")
    );
    assert_eq!(longest_common_prefix(vec![]), String::from(""));
    assert_eq!(
        longest_common_prefix(vec![String::from(""),]),
        String::from("")
    );
}
