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

    let mut signal_sum = 0;
    for cycle in [20, 60, 100, 140, 180, 220] {
        let signal_strength = register[cycle] * cycle as i32;
        println!("{}th is {}", cycle, signal_strength);
        signal_sum += signal_strength;
    } 

    println!("Sum Signal Strength: {}", signal_sum);

}
