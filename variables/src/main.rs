fn main() {
    let tup = (2, 3.4, "a");
    println!("The second element is '{}'", tup.2);

    let arr: [f32; 5] = [1., 2., 3., 4., 5.];

    println!("The second element of arr is '{}'", arr[2]);

    let large_array = [3; 1000];

    println!("The 726th element is '{}'", large_array[726]);

    println!("Please enter an array index for {:?}", arr);

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!("Accessed value {element}")
}
