use std::str::FromStr;

#[derive(Clone)]
struct Monkey {
    items: Vec<i32>,
    add: bool,
    worry_factor: i32,
    divisor: i32,
    inspect_counter: i32,
    true_monkey: usize,
    false_monkey: usize
}

impl FromStr for Monkey {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if !s.starts_with("Monkey") {
            return Err::<Monkey, String>("test".to_string());
        }

        let mut items: Vec<i32> = vec![];
        let mut add = false;
        let mut worry_factor = 0;
        let mut divisor = 0;
        let mut true_monkey = 0 as usize;
        let mut false_monkey = 0 as usize;

        for line in s.lines() {
            if line.starts_with("  Sta") {
                let line = line.strip_prefix("  Starting items: ").expect("Should strip");
                items = line.split(", ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            }
            if line.starts_with("  Ope") {
                let line = line.strip_prefix("  Operation: new = old ").expect("Should strip");
                let (op, factor) = line.split_once(" ").expect("Should split");
                match op {
                    "*" => add = false,
                    "+" => add = true,
                    _ => panic!("Why?")
                }

                match factor {
                    "old" => worry_factor = -1, // Will be handled later as square root
                    _ => {
                        worry_factor = factor.parse::<i32>().expect("Should parse");
                    }
                }
            }
            if line.starts_with("  Te") {
                let line = line.strip_prefix("  Test: divisible by ").expect("Should strip");
                divisor = line.parse::<i32>().expect("Should parse");
            }
            if line.starts_with("    If"){
                let line = line.strip_prefix("    If ").expect("Should strip");
                let (prefix, value) = line.split_once(": ").expect("Should split");
                let value = value.strip_prefix("throw to monkey ").expect("Should strip").parse::<usize>().expect("Should parse");
                match prefix {
                    "true" => true_monkey = value,
                    "false" => false_monkey = value,
                    _ => panic!("Why?")
                }
            }
        }
        let monkey = Monkey{items: items, add: add, worry_factor: worry_factor,
        divisor: divisor, inspect_counter: 0, true_monkey: true_monkey, false_monkey: false_monkey};
        Ok(monkey)
    }
}

impl Monkey {
    fn remove_items(&mut self, items: &[i32]) {

        let mut new_items = vec![];
        for item in self.items.iter() {
            if !items.contains(&item) {
                new_items.push(item.clone());
            }
        }

        self.items = new_items;
    }
    
    fn iter_inspect(&mut self, inspected: i32) {
        self.inspect_counter = self.inspect_counter + inspected;
    }

    fn push(&mut self, item: i32) {
        self.items.push(item);
    }
}

fn main() {

    // Parse Monkeys
    let file_str = include_str!("../input_day11").split("\n\n");
    let mut monkeys: Vec<Monkey> = vec![];

    for str in file_str {
        if let Ok(monkey) = str.parse::<Monkey>() {
            monkeys.push(monkey);
        }
    }
    
    for monkey in monkeys.iter() {
        println!("{:?}", monkey.items);
    }

    // Play Rounds
    for _round in 1..21 {
        println!("Round {}", _round);
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            println!("Monkey {}:", i);
            let mut to_remove = vec![];
            let mut inspected = 0;
            let mut to_add = vec![];

            println!("Monkey {}: {:?}", i, monkey.items);
            for item in monkey.items.iter() {

                println!("  Monkey inspects item with worry level {}", item);
                inspected +=1;
                let mut worry_level;
                if monkey.add {
                    worry_level = monkey.worry_factor + item;
                    println!("    Worry Level is added by {} to {}", monkey.worry_factor, worry_level);
                }
                else {
                    if monkey.worry_factor == -1 {
                        worry_level = item * item;
                        println!("    Worry Level is multiplied by itself to {}", worry_level);
                    }
                    else {
                        worry_level = monkey.worry_factor * item;
                        println!("    Worry level is multiplied by {} to {}", monkey.worry_factor, worry_level);
                    }
                }

                worry_level = worry_level / 3;
                println!("    Worry Level is divided by 3 to {}", worry_level);
                let throw_idx;
                if worry_level % monkey.divisor == 0 {
                    println!("    Current worry level divisible by {}", monkey.divisor);
                    throw_idx = monkey.true_monkey;
                }
                else {
                    println!("    Current worry level is not divisible by {}", monkey.divisor);
                    throw_idx = monkey.false_monkey;
                }
                println!("    Item with worry level {} is thrown to monkey {}", worry_level, throw_idx);
                to_add.push((throw_idx, worry_level.clone()));
                to_remove.push(item.clone());
            }
            monkey.remove_items(&to_remove);
            monkey.iter_inspect(inspected);

            for item in to_add {
                let (monkey_idx, monkey_item) = item;
                monkeys[monkey_idx].push(monkey_item);
            }

        }
        for (i, monkey) in monkeys.iter().enumerate() {
            println!("Monkey {}: {:?}", i, monkey.items);
        }
    }
    
    let mut inspect_counters = monkeys.iter().map(|x| x.inspect_counter).collect::<Vec<i32>>();
    for (i, counter) in inspect_counters.iter().enumerate() {
        println!("Monkey {} inspected items {} times.", i, counter);
    }

    inspect_counters.sort();
    println!("{:?}", inspect_counters);
    let score = inspect_counters[inspect_counters.len()-2..inspect_counters.len()].iter().product::<i32>();
    println!("The score is {}", score);



    
}
