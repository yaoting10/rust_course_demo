fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (_, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..1];
        }
    }
    &s[..]
    // let s1 = "sd";
    // return s1
}