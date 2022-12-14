fn main() {

    let mut register: Vec<i32> = vec![1];

    let lines = include_str!("../input_day10").lines();
    let mut cycle = 0;
    let mut result = 1;

    for line in lines {
        if line.starts_with("noop") {
            register.push(result);

            cycle = cycle + 1;
        }
        else {
            let (op, add) = line.split_once(" ").expect("Should work");
            assert_eq!(op, "addx");
            let add = add.parse::<i32>().expect("Should parse");
            register.push(result);
            register.push(result);
            result += add;

            cycle = cycle + 2;
        }
    }

    // Render the CRT screen

    let mut current_position = 0;
    let mut current_line = vec![];
    
    // Remove first 1, because it was faultfully added in the first part
    for reg in register[1..register.len()].iter() {

        let mut current_pixel = ".";
        let diff = current_position - reg;
        if diff.abs() <= 1 {
            current_pixel = "#";
        }
        
        current_line.push(current_pixel);
        current_position += 1;
        if current_position > 39 {
            current_position = 0;
            println!("{}", current_line.join(""));
            current_line.clear();
        }
    }
}
