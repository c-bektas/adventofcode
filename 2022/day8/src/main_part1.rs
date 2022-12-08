fn visible(tree: &usize, neighbors: &[usize]) -> bool {
    let count = neighbors.iter().filter(|x| x >= &&tree).count();
    return count == 0;
}

fn main() {

    let lines = include_str!("../input_day8").lines();
    let mut rows: Vec<Vec<usize>> = vec![];
    
    // Parse everything
    for line in lines {
        let trees = line.chars().map(|c| c.to_string().parse::<usize>().expect("")).collect::<Vec<usize>>();
        rows.push(trees);
    }

    // Check visibilites
    let mut visibles: i32 = 0;
    for (row_idx, row) in rows.iter().enumerate() {
        for (tree_idx, tree) in row.iter().enumerate() {

            let top = &rows[0..row_idx].iter().map(|x| x[tree_idx]).collect::<Vec<usize>>();
            let bottom = &rows[row_idx+1..rows.len()].iter().map(|x| x[tree_idx]).collect::<Vec<usize>>();

            let left = &row[0..tree_idx];
            let right = &row[tree_idx+1..row.len()];
           
            if visible(tree, top) || visible(tree, bottom) || visible(tree, left) || visible(tree, right) {
                visibles += 1;
            }

        }
    }

    println!("{}", visibles);
}
