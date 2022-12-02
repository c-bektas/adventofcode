// 1 Rock, 2 Paper, 3 Scissors
// Win 6 points, Draw 3, Lose 0
// X lose, Y Draw, Z Win

fn main() {

    let points_rock = 1;
    let points_paper = 2;
    let points_scissors = 3;

    let points_win = 6;
    let points_draw = 3;

    let mut my_points: Vec<i32> = Vec::<i32>::new();

    let lines = include_str!("../input_day2").lines();
    for line in lines {
        let splits = line.split(" ").collect::<Vec<&str>>();

        let opp_hand = splits[0];
        let my_hand = splits[1];

        let mut my_current_points = 0;

        match opp_hand {
            "A" => {
                match my_hand {
                    "X" => {my_current_points += points_scissors;},
                    "Y" => {my_current_points += points_rock + points_draw;},
                    "Z" => {my_current_points += points_paper + points_win;},
                    _ => panic!("Should not happen A"),
                }
            }

            "B" => {
                match my_hand {
                    "X" => {my_current_points += points_rock;},
                    "Y" => {my_current_points += points_paper + points_draw;},
                    "Z" => {my_current_points += points_scissors + points_win;},
                    _ => panic!("Should not happen B"),
                }
            }

            "C" => {
                match my_hand {
                    "X" => {my_current_points += points_paper;},
                    "Y" => {my_current_points += points_scissors + points_draw;},
                    "Z" => {my_current_points += points_rock + points_win;},
                    _ => panic!("Should not happen C"),
                }
            }
            _ => panic!("Should not happen"),


        }

        my_points.push(my_current_points);
    }

    let my_sum_points: i32 = my_points.iter().sum();
    println!("My sum points: {}", my_sum_points);
}
