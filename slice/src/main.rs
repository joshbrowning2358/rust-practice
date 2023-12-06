fn main() {
    let mut my_string = String::from("this is a test");
    let ans = first_word(&my_string);
    println!("Slice is {ans}");

    my_string.clear();
    println!("Slice is {ans}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
