fn main() {
    multiple_references()
}

fn multiple_references() {
    let mut s = String::from("hello");

    let s1: &String = &s;
    let s2: &String = &s;
    let s21: &String = &s1;

    println!("String is {s1} and {s2}");

    let s3: &mut String = &mut s;
    s3.push_str(", world");
    println!("String reference is {s3}");
    println!("String is {s}");
}

fn initial_string() {
    let s: &str = "hello";
    let mut s2: String = String::from("hello");
    s2.push_str(", world!");
    println!("String is {s2}");
}

fn string2() {
    let x = 5;
    let mut y = x;

    y += 1;
    println!("x is {x} and y is {y}");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 is {s1}");
}

fn practice() {
    let s = String::from("hello");

    takes_ownership(s.clone());

    let x = 5;

    make_copy(x);

    println!("x is still {x}");
    println!("String is {s}");
}

fn takes_ownership(some_string: String) {
    println!("String is {some_string}");
}

fn make_copy(val: i32) {
    println!("Int is {val}");
}

fn practice2() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("s1 is {s1} and s3 is {s3}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn references1() {
    let mut s = String::from("hello!");
    let len = calculate_len(&s);
    steal(&mut s);

    println!("String {s} has length {len}!");
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn steal(s: &mut String) {
    s.push_str(", world");
}
