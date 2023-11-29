fn loop1() {
    let x: i32 = 123;

    // if x < 10{
    if x % 4 == 0 {
        println!("Divisible by 4!");
    } else if x % 3 == 0 {
        println!("Divisible by 3");
    } else if x % 2 == 0 {
        println!("Divisible by 2!");
    } else {
       println!("Maybe you have a prime?")
    }

    let _number = if x % 4 == 0 {5} else {6};

    let mut counter: i32 = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result was {result}");

}

fn loop2() {
    let mut count: i32 = 0;
    'outer: loop {
        println!("Outer loop count {count}");
        let mut remaining: i32 = 10;

        'inner: loop {
            println!("Remaining {remaining}");
            if remaining == 5 {
                break;
            } else if count == 2 {
                break 'outer;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("Done, count is {count}");
}

fn while_loop() {
    let mut number: i32 = 0;

    while number < 10 {
        println!("Number is {number}");
        number += 1;
    }
}

fn for_loop() {
    let a: [i32; 6] = [1, 2, 3, 4, 5, 100];
    for val in a {
        println!("Value is {val}");
    }

    for val in (1..10).rev() {
        println!("Now value is {val}");
    }
}

fn main() {
    for_loop()
}
