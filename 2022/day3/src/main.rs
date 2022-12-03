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
    fn new(group: &[&str]) -> ElfGroup {
        if group.len() != 3 {
            panic!("Should be 3");
        }
        else {
            let common: char = find_common_char(group[0].to_string(), group[1].to_string(), group[2].to_string()).unwrap();
            let priority = get_priority(common);

            return ElfGroup{ common: common.to_owned(), priority: priority.to_owned()};
        }
    }

}


fn main() {

    let lines: Vec<&str> = include_str!("../input_day3").lines().collect();
    let elf_groups = lines.chunks(3).map(|x| ElfGroup::new(x));

    let sum_prio: i32 = elf_groups.map(|x| x.priority).sum();
    println!("Sum Priority: {}", sum_prio);
}
