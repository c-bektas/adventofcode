use std::str::FromStr;
use std::ops::Range;
use std::collections::HashSet;


struct Assignment {
    intersects: bool,
    overlaps: bool
}

impl FromStr for Assignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Assignment, Self::Err> {
    
        let (range1, range2) = s.split_at(s.find(",").unwrap());
        let ranges = vec![range1.to_string(), range2.to_string()];

        let mut hashes: Vec<HashSet<i32>> = vec![];
        for range in ranges {

            let (r1, r2) = range.split_at(range.find("-").unwrap());
            let r1 = r1.to_string().replace(",", "").replace("-", "").parse::<i32>().unwrap();
            let r2 = r2.to_string().replace(",", "").replace("-", "").parse::<i32>().unwrap();
            let ran: Vec<i32> = Range{ start: r1, end: r2+1 }.collect();

            let hash_set: HashSet<i32> = HashSet::from_iter(ran.iter().cloned());
            hashes.push(hash_set);
        }

        let intersects = hashes[0].is_subset(&hashes[1]) || hashes[1].is_subset(&hashes[0]);
        let overlaps = hashes[0].intersection(&hashes[1]).count() > 0;

        let assignment = Assignment{overlaps: overlaps.to_owned(), intersects: intersects.to_owned()};
        Ok(assignment)
    }
}


fn main() {

    let assignments: Vec<Assignment> = include_str!("../input_day4").lines().map(|s| Assignment::from_str(s).unwrap()).collect::<Vec<Assignment>>();

    let intersects = assignments.iter().filter(|a| a.intersects).count();
    let overlaps = assignments.iter().filter(|a| a.overlaps).count();
    println!("Intersects: {}, Overlaps: {}", intersects, overlaps);

}
