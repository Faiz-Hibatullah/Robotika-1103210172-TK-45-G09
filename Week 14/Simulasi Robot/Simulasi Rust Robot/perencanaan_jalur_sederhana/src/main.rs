use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: (usize, usize),
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star(grid: &Vec<Vec<u8>>, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = vec![vec![None; grid[0].len()]; grid.len()];
    let mut cost_so_far = vec![vec![usize::MAX; grid[0].len()]; grid.len()];

    open_set.push(Node { position: start, cost: 0 });
    cost_so_far[start.0][start.1] = 0;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(Node { position, .. }) = open_set.pop() {
        if position == goal {
            let mut path = vec![];
            let mut current = goal;
            while let Some(prev) = came_from[current.0][current.1] {
                path.push(current);
                current = prev;
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        for &(dx, dy) in &directions {
            let next = (
                position.0 as isize + dx,
                position.1 as isize + dy,
            );

            if next.0 >= 0
                && next.1 >= 0
                && (next.0 as usize) < grid.len()
                && (next.1 as usize) < grid[0].len()
            {
                let next = (next.0 as usize, next.1 as usize);
                if grid[next.0][next.1] == 1 {
                    continue;
                }

                let new_cost = cost_so_far[position.0][position.1] + 1;
                if new_cost < cost_so_far[next.0][next.1] {
                    cost_so_far[next.0][next.1] = new_cost;
                    open_set.push(Node {
                        position: next,
                        cost: new_cost,
                    });
                    came_from[next.0][next.1] = Some(position);
                }
            }
        }
    }

    None
}

fn main() {
    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 0],
    ];

    let start = (0, 0);
    let goal = (4, 4);

    if let Some(path) = a_star(&grid, start, goal) {
        println!("Path: {:?}", path);
    } else {
        println!("No path found.");
    }
}
