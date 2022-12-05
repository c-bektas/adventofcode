use std::ops::Range;

fn main() {

    let file_str = include_str!("../input_day5");

    let find_str = "1   2   3   4   5   6   7   8   9 \n";
    let (stacks, ops) = file_str.split_at(file_str.find(find_str).unwrap());

    let stacks = stacks.to_string().replace(find_str, "");
    let ops = ops.to_string().replace(find_str, "");

    let mut heaps: Vec<Vec<String>> = Range{start: 0, end: 9}.map(|_x| Vec::new()).collect();

    // Create the heaps
    for strip in stacks.lines().rev() {
        let strip = strip.replace("    ", "X,").replace("[", "").replace("]", ",").replace(" ", "");
        let strip: Vec<String> = strip.split(",").filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>();

        if !strip.is_empty() {
            for (i, elem) in strip.iter().enumerate() {
                if !elem.eq("X") {
                    heaps[i].push(elem.to_string());
                }
            }
        }
    }

    // Operate the cranes
    for op in ops.lines(){
        let op = op.replace("move ", "").replace(" from ", ",").replace(" to ", ",").split(",").map(|x| x.parse::<usize>().unwrap_or(10000)).collect::<Vec<usize>>();
        if !op.contains(&10000){
            let mut n = op[0];
            let from = op[1] - 1;
            let to = op[2] - 1;

            //Part 1
            /*
            while n > 0 {
                let last = heaps[from].pop().unwrap();
                heaps[to].append(last);
                n = n + 1;
            }
            */

            let trunc_n = heaps[from].len() - n;
            let mut last_n = heaps[from].as_slice()[trunc_n..].to_vec();
            heaps[from].truncate(trunc_n);
            heaps[to].append(&mut last_n);
            }
        }

    //Show top elements
    for heap in heaps {
        print!("{}", heap.last().unwrap());
    }
}
        

