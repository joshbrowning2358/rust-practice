use advent_of_code_2024::file_reader;

#[derive(Clone)]
struct Registers {
    a: i128,
    b: i128,
    c: i128,
}

impl Registers {
    fn new() -> Registers {
        Registers{a: 0, b: 0, c: 0}
    }
}

fn main() {
    let full_path = file!();
    let (_, mut file_name) = full_path.rsplit_once('/').unwrap();
    (file_name, _) = file_name.split_once('.').unwrap();
    let file_path = format!("data/{file_name}/input.txt");

    let mut ans = part_a(&file_path);
    println!("Answer to {file_name} a is {ans};");

    ans = part_b(&file_path);
    println!("Answer to {file_name} b is {ans};");
}

fn part_a(file_path: &str) -> String {
    let (program, mut registers) = parse_input(file_path);
    let output = run_program(&program, registers);
    return output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
}

fn part_b(file_path: &str) -> String {
    // 1650854000 is too low
    let (program, mut registers) = parse_input(file_path);

    //println!("Just running the program");
    let mut a_0: i128 = 1;
    //for i in 0..500 {
    //    registers.a = a_0;
    //    let output = run_program(&program, registers.clone());
    //    println!("Output for {a_0} is {output:?}");
    //    a_0 += 1;
    //}

    // a_0 = 1;
    'main: loop {
        println!("Trying {a_0}!");
        registers.a = a_0;
        let output = run_program(&program, registers.clone());
        if output == program {
            break
        }
        //println!("  Result for {a_0} is {output:?}");
        //if output == program[(program.len() - match_len as usize - 1)..] {
        //    println!("Partial match at {a_0} with {output:?}!");
        //    a_0 *= 8;
        //    match_len += 1;
        //    continue
        //}
        println!("Got result {output:?}");
        for (i, (ans, tar)) in output.iter().rev().zip(program.iter().rev()).enumerate() {
            if ans != tar {
                a_0 += 8_i128.pow((output.len() - i - 1) as u32);
                continue 'main
            }
        }
        a_0 *= 8;
        //if a_0 > 100000 {
        //    break
        //}
    }

    //registers.a = a_0;
    //let output = run_program(&program, registers.clone());
    //println!("Result is {output:?}");
    //println!(" Input is {program:?}");

    return a_0.to_string();
}

fn parse_input(file_path: &str) -> (Vec<i32>, Registers) {
    let mut program = Vec::new();
    let mut registers: Registers = Registers::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                if row.len() == 0 {
                    continue
                }
                let (name, val) = row.split_once(": ").unwrap();
                match name {
                    "Register A" => {registers.a = val.parse::<i128>().unwrap();}
                    "Register B" => {registers.b = val.parse::<i128>().unwrap();}
                    "Register C" => {registers.c = val.parse::<i128>().unwrap();}
                    "Program" => {program = val.split(',').map(|x| x.parse::<i32>().unwrap()).collect();}
                    _ => {panic!("Invalid name {name}!");}
                }
            }
        }
    }

    return (program, registers)
}

fn run_program(program: &Vec<i32>, mut registers: Registers) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < program.len() - 1 {
        let opcode = program[i];
        let mut combo_operand = program[i + 1] as i128;
        if combo_operand > 3 {
            combo_operand = match combo_operand {
                4 => registers.a,
                5 => registers.b,
                6 => registers.c,
                _ => panic!("Found invalid operand {combo_operand}!")
            };
        }
        let literal_operand = program[i + 1] as i128;

        match opcode {
            0 => {
                let result = (registers.a as f64) / 2_f64.powf(combo_operand as f64);
                registers.a = result as i128;
            }
            1 => {registers.b ^= literal_operand as i128;}  // Bitwise XOR
            2 => {registers.b = (combo_operand % 8) as i128;}
            3 => {
                if registers.a != 0 {
                    i = literal_operand as usize;
                    //println!("Continuing");
                    continue
                }
            }
            4 => {registers.b ^= registers.c;}
            5 => {output.push((combo_operand % 8) as i32);}
            6 => {
                let result = (registers.a as f64) / 2_f64.powf(combo_operand as f64);
                registers.b = result as i128;
            }
            7 => {
                let result = (registers.a as f64) / 2_f64.powf(combo_operand as f64);
                registers.c = result as i128;
            }
            _ => {panic!("Invalid opcode {opcode}!");}
        }
        i += 2;
    }

    return output
}
