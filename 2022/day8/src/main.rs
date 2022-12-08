fn calc_score(tree: &usize, neighbors: &[usize], reverse: bool) -> i32 {
    let mut score = 0;
    if !reverse {
        for (i, neighbor) in neighbors.iter().enumerate() {
            if neighbor >= tree || i == neighbors.len()-1 {
                score = i + 1;
                break;
            }
        }

    }

    else {

        for (i, neighbor) in neighbors.iter().rev().enumerate() {
            if neighbor >= tree  || i == neighbors.len()-1 {
                score = i + 1;
                break;
            }
        }
    }
    return score as i32;
}

fn main() {

    let lines = include_str!("../input_day8").lines();
    let mut rows: Vec<Vec<usize>> = vec![];
    
    // Parse everything
    for line in lines {
        let trees = line.chars().map(|c| c.to_string().parse::<usize>().expect("")).collect::<Vec<usize>>();
        rows.push(trees);
    }

    // Calculate scores
    let mut highest_score = 0;
    for (row_idx, row) in rows.iter().enumerate() {
        for (tree_idx, tree) in row.iter().enumerate() {

            let top = &rows[0..row_idx].iter().map(|x| x[tree_idx]).collect::<Vec<usize>>();
            let bottom = &rows[row_idx+1..rows.len()].iter().map(|x| x[tree_idx]).collect::<Vec<usize>>();

            let left = &row[0..tree_idx];
            let right = &row[tree_idx+1..row.len()];
           
            let score = calc_score(tree, top, true) * calc_score(tree, bottom, false) * calc_score(tree, left, true) * calc_score(tree, right, false);
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    println!("{}", highest_score);
}
