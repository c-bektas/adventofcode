use std::collections::HashSet;

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }

    fn sub(&mut self, other: &Point) {
        self.x -= other.x;
        self.y -= other.y;
    }

    fn set_to_one(&mut self) {
        if self.x >= 1 {
            self.x = 1;
        }
        if self.y >= 1 {
            self.y = 1;
        }

        if self.x <= -1 {
            self.x = -1;
        }
        if self.y <= -1 {
            self.y = -1;
        }
    }

    fn is_adjacent(&self, other: &Point) -> bool {
        let mut dx = self.x - other.x;
        if dx < 0 {
            dx = dx * -1;
        }
        let mut dy = self.y - other.y;
        if dy < 0 {
            dy = dy * -1;
        }

        return dx <= 1 && dy <= 1;

    }

    fn as_tuple(&self) -> (i32, i32) {
        return (self.x, self.y);
    }
}

#[derive(Clone)]
struct Rope {
    head: Point,
    tail: Point,
    tail_visited: HashSet<(i32, i32)>
}

impl Rope {

    fn tail_too_far(&self) -> bool {
        return !self.head.is_adjacent(&self.tail);
    }

    fn move_tail(&mut self) {
        let mut mov = Point{x: self.head.x, y: self.head.y};
        mov.sub(&self.tail);
        mov.set_to_one();
        self.tail.add(&mov);
        self.tail_visited.insert(self.tail.as_tuple());
    }

    fn move_head(&mut self, action: &Point){
        self.head.add(&action);
        if self.tail_too_far() {
            self.move_tail();
        }
    }

    fn set_head(&mut self, head: &Point) {
        self.head = *head;
        if self.tail_too_far() {
            self.move_tail();
        }
    }

    fn get_visited(&self) -> usize {
        return self.tail_visited.len();
    }

}
fn main() {

    let mut ropes = vec![];
    for _ in 0..9 {
        let rope = Rope{head: Point{x: 0, y:0}, tail: Point{x: 0, y: 0}, tail_visited: HashSet::new()};
        ropes.push(rope);
    }
    let lines = include_str!("../input_day9").lines();
    for line in lines {
        let (direction, steps) = line.split_once(" ").expect("Should split");
        let steps = steps.to_string().parse::<i32>().expect("Should parse");

        let mut mov_x = 0;
        let mut mov_y = 0;
        match direction {
            "R" => mov_x = 1,
            "L" => mov_x = -1,
            "U" => mov_y = 1,
            "D" => mov_y = -1,
            _ => panic!("Why?")
        }

        let mut actions = vec![];
        for _ in 0..steps {
            actions.push(Point{x: mov_x, y: mov_y});
        }

        // The tail of the previous is the head of the next rope
        for action in actions {
            let mut next_head = Point{x: 0, y: 0};
            for (i, rope) in ropes.iter_mut().enumerate() {
                if i > 0 {
                    rope.set_head(&next_head);
                }
                else {
                    rope.move_head(&action);
                }
                next_head = rope.tail.clone();
            }
        }
    }
    println!("Tail visited: {}", ropes.last().expect("").get_visited() + 1);
}
