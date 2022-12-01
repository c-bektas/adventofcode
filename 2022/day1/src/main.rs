use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Elf {
    idx: i32,
    calories: i32
}

impl Elf {
    pub fn new(idx: i32, calories: i32) -> Self {
        Elf {
            idx,
            calories
        }
    }
}

fn main() {
    let input_file = "input_day1";

    let mut elf_list: Vec<Elf> = Vec::new();
    let mut current_calories: i32 = 0;
    let mut elf_index: i32 = 0;

    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(ip) = line {
                match ip.parse::<i32>() {
                    Ok(n) => current_calories = current_calories + n, 
                    Err(_) => {
                        let elf = Elf::new(elf_index, current_calories);
                        elf_list.push(elf);
                        current_calories = 0;
                        elf_index = elf_index + 1;
                    }
                }
            }
        }
    }

    elf_list.sort_by(|a, b| b.calories.cmp(&a.calories));

    let mut sum_calories: i32 = 0;
    println!("Elf Calorie Ranking");

    for i in 0..3 {
        match elf_list.get(i) {
            Some(elf) => {
                println!("Elf {} with {} calories", elf.idx, elf.calories);
                sum_calories = sum_calories + elf.calories;
            }
            None => println!("Error")
        }
    }
    println!("Sum Calories: {}", sum_calories);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
