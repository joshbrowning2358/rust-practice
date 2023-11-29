fn main() {
    // TODO: Get user input
    let n: i32 = 50;
    println!("The {n}th fibonacci number is {}", fibonacci_loop(n));
    println!("The {n}th fibonacci number is {}", fibonacci(n));
}

fn fibonacci(n: i32) -> u64 {
    if n == 1 {
        1u64
    } else if n == 2 {
        1u64
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn fibonacci_loop(n: i32) -> u128 {
    if n == 1 {
        1u128
    } else if n == 2 {
        1u128
    } else {
        let mut a: u128 = 1;
        let mut b: u128 = 1;
        for _iter in 3..(n + 1) {
            let x: u128 = a + b;
            a = b;
            b = x;
        }
        b
    }
}
