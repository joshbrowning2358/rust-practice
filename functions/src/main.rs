fn main() {
    println!("Hello world!");

    another_function(5, 'h');
    println!("One more than 5 is {}", plus_one(5));
}

fn another_function(x: i32, unit: char) {
    println!("The value of x is {x}{unit}");
}

fn plus_one(x: i32) -> i32 {
    return x + 1
}
