use std::str::FromStr;
use std::string::ParseError;


fn find_common_char(s1: String, s2: String) -> Result<char, String> {

    let mut common: char = 'ä';

    for character in s1.chars() {
        if s2.contains(character) {
            common = character;
            break;
        }
    }
    if common.eq(&'ä') {
        return Err("No common character found".to_string());
    }
    Ok(common.to_owned())
}

fn get_priority(character: char) -> i32 {
    let code = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut prio = 0;
    for pos in code.chars() {
        prio += 1;
        if character.eq(&pos){
            break;
        }
    }
    return prio
}


struct Rucksack {
    compartment_1: String,
    compartment_2: String,
    common: char,
    priority: i32
}

impl FromStr for Rucksack {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let priority = 23;

        let mut mid_f: f64 = s.len() as f64;
        mid_f = mid_f / 2.0;
        mid_f = mid_f.ceil();
        let mid: usize = mid_f as usize;
        
        let (compartment_1, compartment_2) = s.split_at(mid);
        let common = find_common_char(compartment_1.to_string(), compartment_2.to_string()).unwrap();

        let priority = get_priority(common);

        Ok(Rucksack { compartment_1: compartment_1.to_owned(), compartment_2: compartment_2.to_owned(), common: common.to_owned(), priority: priority })
    }    
}

fn main() {

    let rucksacks = include_str!("../input_day3").lines().map(|x| Rucksack::from_str(x).unwrap());

    let sum_prio: i32 = rucksacks.map(|x| x.priority).sum();
    println!("Sum Priority: {}", sum_prio);
}
