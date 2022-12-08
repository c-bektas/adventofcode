fn calc_score(tree: &usize, neighbors: &Vec<usize>) -> i32 {
    let mut score = 0;
    for (i, neighbor) in neighbors.iter().enumerate() {
        if neighbor >= tree || i == neighbors.len()-1 {
            score = i + 1;
            break;
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

            // Reverse top and left slices for right point of view for tree
            let top = &rows[0..row_idx].iter().map(|x| x[tree_idx]).rev().collect::<Vec<usize>>();
            let bottom = &rows[row_idx+1..rows.len()].iter().map(|x| x[tree_idx]).collect::<Vec<usize>>();

            let left = &row[0..tree_idx].to_vec().iter().rev().map(|x| x.to_owned()).collect::<Vec<usize>>();
            let right = &row[tree_idx+1..row.len()].to_vec();
           
            let score = calc_score(tree, top) * calc_score(tree, bottom) * calc_score(tree, left) * calc_score(tree, right);
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    println!("{}", highest_score);
}
