use std::fs;

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: String,
    test_divisor: i64,
    if_true: usize,
    if_false: usize,
    inspected_items: i64,
}

fn main() {
    println!("part one: {}", monkey_business(20, true));
    println!("part two: {}", monkey_business(10000, false));
}

fn monkey_business(rounds: i32, divide_worry: bool) -> i64 {
    let file_contents = read_file("input.txt");
    let blocks = file_contents.split("\n\n");

    let mut monkeys: Vec<Monkey> = Vec::new();
    for block in blocks {
        monkeys.push(parse_monkey(&block));
    }

    let modulo: i64 = monkeys.iter().map(|m| m.test_divisor).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let operation = monkeys[i].operation.as_str();
                let mut worry: i64 = monkeys[i].items[j];
                let multiplier = get_operation_multiplier(operation, monkeys[i].items[j]);
                let sum = get_operation_sum(operation, monkeys[i].items[j]);

                worry += sum;
                worry *= multiplier;
                if divide_worry {
                    worry /= 3;
                }

                worry %= modulo;

                let destination = if worry % monkeys[i].test_divisor == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };

                monkeys[destination].items.push(worry);
                monkeys[i].inspected_items += 1;
            }
            monkeys[i].items = Vec::new();
        }
    }

    monkeys.sort_by(|a, b| b.inspected_items.cmp(&a.inspected_items));
    return monkeys[0].inspected_items * monkeys[1].inspected_items;
}

fn parse_monkey(block: &str) -> Monkey {
    let mut lines = block.split("\n");
    lines.next().unwrap(); // useless header
    let items: Vec<i64> = lines.next().unwrap().split(":").collect::<Vec<&str>>()[1]
        .split(",")
        .map(|i| i.trim().parse().unwrap())
        .collect::<Vec<i64>>();

    let operation = lines.next().unwrap().split(":").collect::<Vec<&str>>()[1]
        .trim()
        .to_string();

    let test_divisor: i64 = lines.next().unwrap().split("by").collect::<Vec<&str>>()[1]
        .trim()
        .parse()
        .unwrap();

    let if_true: usize = lines.next().unwrap().split("monkey").collect::<Vec<&str>>()[1]
        .trim()
        .parse()
        .unwrap();

    let if_false: usize = lines.next().unwrap().split("monkey").collect::<Vec<&str>>()[1]
        .trim()
        .parse()
        .unwrap();

    return Monkey {
        items: items,
        operation: operation,
        test_divisor: test_divisor,
        if_true: if_true,
        if_false: if_false,
        inspected_items: 0,
    };
}

fn get_operation_multiplier(operation: &str, item: i64) -> i64 {
    if operation.contains("*") {
        let parts = operation.split("*").collect::<Vec<&str>>();
        let multiplier = parts[1].trim();
        if multiplier == "old" {
            return item;
        }
        return multiplier.parse::<i64>().unwrap();
    } else {
        return 1;
    }
}

fn get_operation_sum(operation: &str, item: i64) -> i64 {
    if operation.contains("+") {
        let parts = operation.split("+").collect::<Vec<&str>>();
        let operand = parts[1].trim();
        if operand == "old" {
            return item;
        }
        return operand.parse::<i64>().unwrap();
    } else {
        return 0;
    }
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Cannot read file");
}
