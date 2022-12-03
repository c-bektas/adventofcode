fn find_common_char(s1: String, s2: String, s3: String) -> Result<char, String> {

    let mut common: char = 'ä';

    for character in s1.chars() {
        if s2.contains(character) && s3.contains(character) {
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


struct ElfGroup {
    common: char,
    priority: i32
}

impl ElfGroup {
    fn new(group: Vec<String>) -> ElfGroup {
        if group.len() != 3 {
            panic!("Should be 3");
        }
        else {
            return ElfGroup{ common: 'a', priority: 1};
        }
    }

}

fn main() {

    // TODO: Split in groups of three and input in new ElfGroups
    let elf_groups = include_str!("../input_day3").lines();

    let sum_prio: i32 = elf_groups.map(|x| x.priority).sum();
    println!("Sum Priority: {}", sum_prio);
}
