use std::collections::HashMap;


fn main() {
   let lines = include_str!("../input_day7").lines();
   let mut folders: HashMap<String, i128> = HashMap::new();
   let mut path = vec![];
   for line in lines {

       if line.starts_with("$ cd") && !line.eq("$ cd ..") {
           let line = line.strip_prefix("$ ").expect("Should not happen");
           let (_cd, folder) = line.split_once(" ").expect("Should not happen");
           path.push(folder.to_owned());
           let key = path.clone().join("/");
           folders.insert(key, 0);
       }

       if line.starts_with("$ cd ..") {
           path.pop();
       }

       if !line.starts_with("$") && !line.starts_with("dir") {
           let key = path.clone().join("/");
           let (size, _name) = line.split_once(" ").expect("Should not happen");
           let size = size.parse::<i128>().expect("Should not happen");
           let current_size = folders.get(&key).expect("Should be in here");
           let new_size = current_size + size;
           folders.insert(key, new_size);

           if path.len() > 1 {
               let parent_key = path[0..path.len()-1].join("/");
               let current_size = folders.get(&parent_key).expect("Should be in here");
               let new_size = current_size + size;
               folders.insert(parent_key, new_size);
           }

       }
   }

   let mut sum: i128 = 0;
   for (_folder, size) in &folders {
       if size <= &100000 {
           sum += size;
       }
   }
   println!("{}", sum);
}
