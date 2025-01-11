use crate::file_reader;
use crate::puzzle_solver;

pub struct Puzzle11;

#[derive(Clone, Debug)]
struct Monkey {
    id: i8,
    items: Vec<u128>,
    op: char,
    factor: u128,
    divisor: u128,
    true_monkey: usize,
    false_monkey: usize
}

impl puzzle_solver::Solver for Puzzle11 {
    fn part_1(&self, file_path: &str) -> String {
        let mut monkeys = parse(file_path);

        let mut monkey_cnts: Vec<u32> = vec![0; monkeys.len()];
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let monkey = monkeys[i].clone();
                for item in monkey.items {
                    monkey_cnts[i] += 1;
                    let new_item = match monkeys[i].op {
                        '*' => item * monkeys[i].factor / 3,
                        '+' => (item + monkeys[i].factor) / 3,
                        '^' => item * item / 3,
                        _ => panic!("Invalid char {}!", monkeys[i].op),
                    };
                    if new_item % monkeys[i].divisor == 0 {
                        monkeys[monkey.true_monkey].items.push(new_item);
                    } else {
                        monkeys[monkey.false_monkey].items.push(new_item);
                    }
                }
                monkeys[i].items = vec![];
            }
        }
        monkey_cnts.sort_by(|a, b| b.cmp(a));
        let ans = monkey_cnts[0] * monkey_cnts[1];
        return ans.to_string()
    }

    fn part_2(&self, file_path: &str) -> String {
        let mut monkeys = parse(file_path);
        let mut modulo = 1;
        for i in 0..monkeys.len() {
            modulo *= monkeys[i].divisor;
        }

        let mut monkey_cnts: Vec<u64> = vec![0; monkeys.len()];
        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                let monkey = monkeys[i].clone();
                for item in monkey.items {
                    monkey_cnts[i] += 1;
                    let new_item = match monkeys[i].op {
                        '*' => (item * monkeys[i].factor) % modulo,
                        '+' => (item + monkeys[i].factor) % modulo,
                        '^' => (item * item) % modulo,
                        _ => panic!("Invalid char {}!", monkeys[i].op),
                    };
                    if new_item % monkeys[i].divisor == 0 {
                        monkeys[monkey.true_monkey].items.push(new_item);
                    } else {
                        monkeys[monkey.false_monkey].items.push(new_item);
                    }
                }
                monkeys[i].items = vec![];
            }
        }
        monkey_cnts.sort_by(|a, b| b.cmp(a));
        let ans = monkey_cnts[0] * monkey_cnts[1];
        return ans.to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("95472", "17926061332")
    }

    fn name(&self) -> &'static str {
        "Day 11: Monkey in the Middle"
    }
}

fn parse(file_path: &str) -> Vec<Monkey>{
    let mut result: Vec<Monkey> = vec![];

    let mut new_monkey = Monkey{id: 0, items: vec![], op: ' ', factor: 0, divisor: 0, true_monkey: 0, false_monkey: 0};
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                if row.starts_with("Monkey") {
                    let (_, id) = row.split_once(" ").unwrap();
                    new_monkey.id = id[0..1].parse::<i8>().unwrap();
                } else if row.starts_with("  Starting") {
                    let (_, items) = row.split_once(": ").unwrap();
                    new_monkey.items = items.split(", ").map(|x| x.parse::<u128>().unwrap()).collect();
                } else if row.starts_with("  Operation") {
                    if row.contains("+") {
                        new_monkey.op = '+';
                    } else if row.contains("* old") {
                        new_monkey.op = '^';
                    } else {
                        new_monkey.op = '*';
                    }
                    if row.contains("* old") {
                        new_monkey.factor = 2;
                    } else if row.contains("*") {
                        new_monkey.factor = row.split_once("* ").unwrap().1.parse::<u128>().unwrap();
                    } else {
                        new_monkey.factor = row.split_once("+ ").unwrap().1.parse::<u128>().unwrap();
                    }
                } else if row.starts_with("  Test") {
                    new_monkey.divisor = row.split_once("by ").unwrap().1.parse::<u128>().unwrap();
                } else if row.starts_with("    If true") {
                    new_monkey.true_monkey = row.split_once("monkey ").unwrap().1.parse::<usize>().unwrap();
                } else if row.starts_with("    If false") {
                    new_monkey.false_monkey = row.split_once("monkey ").unwrap().1.parse::<usize>().unwrap();
                } else {
                    result.push(new_monkey.clone());
                }
            }
        }
    }
    result.push(new_monkey.clone());
    return result
}
