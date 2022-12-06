use std::collections::HashSet;

fn main() {

    let data = include_str!("../input_day6");
    let chars = data.chars().collect::<Vec<char>>();

    let mut processed_chars = 0;
    let unique_len = 14; // 4 for Part 1

    for start in 0..chars.len() {
        let end = start + unique_len;
        if !(end > chars.len()-1) {
            let chunk = chars[start..end].iter();
            assert_eq!(chunk.len(), unique_len);

            let set: HashSet<char> = HashSet::from_iter(chunk.cloned());
            if set.len() >= unique_len {
                processed_chars = end;
                break;
            }
        }
    }

    println!("Unique chars after {}", processed_chars);
    
}
