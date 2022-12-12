use std::any::Any;
use std::cmp::{Ordering, min_by};
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal { return Some(cost); }

        // Important as we may have already found a better way
        if cost > dist[position] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Node {
    id: usize,
    height: char
}

fn get_height(c: &char) -> i32 {
    let code = "abcdefghijklmnopqrstuvwxyz";

    let mut height = 0; for pos in code.chars() {
        height += 1;
        if c.eq(&pos){
            break;
        }
    }
    return height
}

fn main() {
    // Parse everything
    let rows_str = include_str!("../input_day12").lines().collect::<Vec<&str>>();
    let mut rows = vec![];
    let default = (10000, 10000);
    let mut starts: Vec<(usize, usize)> = vec![];
    let mut goal: (usize, usize) = default;
    let mut node_id: usize = 0;
    for (x, row_x) in rows_str.iter().enumerate() {
        let row = row_x.replace("S", "a");
        let mut col = vec![];
        for c in row.chars() {
            col.push(Node {id: node_id, height: c });
            node_id += 1;
        }
        let start_idx = col.iter().position(|&r| r.height == 'a');
        let end_idx = col.iter().position(|&r| r.height == 'E');
        match start_idx {
            Some(start_idx) => {
                starts.push((x, start_idx));
            }
            None => {}
        }

        match end_idx {
            Some(end_idx) => {
                goal = (x, end_idx);
            }
            None => {}
        }
        rows.push(col);
    }

    assert_ne!(goal, default);

    let mut graph = vec![];

    for x in 0..rows.len() {
        let col = &rows[x];
        for y in 0..col.len() {
            let mut edges: Vec<Edge> = vec![];
            let mut to_edge: Vec<Node> = vec![];
            let neighbor = x as i32 - 1;
            if neighbor >= 0 {
                to_edge.push(rows[neighbor as usize][y]);
            }
            let neighbor = x + 1;
            if neighbor < rows.len() {
                to_edge.push(rows[neighbor][y]);

            }
            let neighbor = y as i32 - 1;
            if neighbor >= 0 {
                to_edge.push(rows[x][neighbor as usize]);
            }
            let neighbor = y + 1;
            if neighbor < col.len() {
                to_edge.push(rows[x][neighbor]);
            }

            for node in to_edge {
                let height_this = get_height(&rows[x][y].height);
                let height_other = get_height(&node.height);
                let passable = height_this >= height_other || (height_other - height_this <= 1 && height_other - height_this >= 0);
                if passable {
                    let edge = Edge {node: node.id, cost: 1};
                    edges.push(edge);
                }
            }
            graph.push(edges);
        }
    }

    assert_eq!(graph.len(), node_id);

    let mut min_shortest = 100000000;

    for start in starts {
        let shortest = shortest_path(&graph, rows[start.0][start.1].id, rows[goal.0][goal.1].id);
        match shortest {
            Some(shortest) => {
                if shortest < min_shortest {
                    min_shortest = shortest;
                }
            }
            None => {}
        }
    }
    println!("Min shortest path: {}", min_shortest);

}
