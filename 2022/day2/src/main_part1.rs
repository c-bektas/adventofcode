// 1 Rock, 2 Paper, 3 Scissors
// Win 6 points, Draw 3, Lose 0

fn main() {

    let points_rock = 1;
    let points_paper = 2;
    let points_scissors = 3;

    let points_win = 6;
    let points_draw = 3;

    let mut opp_points: Vec<i32> = Vec::<i32>::new();
    let mut my_points: Vec<i32> = Vec::<i32>::new();

    let lines = include_str!("../input_day2").lines();
    for line in lines {
        let splits = line.split(" ").collect::<Vec<&str>>();

        let opp_hand = splits[0];
        let my_hand = splits[1];

        let mut opp_current_points = 0;
        let mut my_current_points = 0;

        match opp_hand {
            "A" => {
                opp_current_points += points_rock;
                match my_hand {
                    "X" => {my_current_points += points_rock + points_draw; opp_current_points += points_draw},
                    "Y" => {my_current_points += points_paper + points_win;},

                    "Z" => {my_current_points += points_scissors; opp_current_points += points_win},
                    _ => panic!("Should not happen A"),
                }
            }

            "B" => {
                opp_current_points += points_paper;
                match my_hand {
                    "X" => {my_current_points += points_rock; opp_current_points += points_win},
                    "Y" => {my_current_points += points_paper + points_draw; opp_current_points += points_draw},
                    "Z" => {my_current_points += points_scissors + points_win;},
                    _ => panic!("Should not happen B"),
                }
            }

            "C" => {
                opp_current_points += points_scissors;
                match my_hand {
                    "X" => {my_current_points += points_rock + points_win;},
                    "Y" => {my_current_points += points_paper; opp_current_points += points_win},
                    "Z" => {my_current_points += points_scissors + points_draw; opp_current_points += points_draw},
                    _ => panic!("Should not happen C"),
                }
            }
            _ => panic!("Should not happen"),


        }

        my_points.push(my_current_points);
        opp_points.push(opp_current_points);
    }

    let my_sum_points: i32 = my_points.iter().sum();
    let opp_sum_points: i32 = opp_points.iter().sum();


    println!("My sum points: {}, Opponent sum points: {}", my_sum_points, opp_sum_points);
}
