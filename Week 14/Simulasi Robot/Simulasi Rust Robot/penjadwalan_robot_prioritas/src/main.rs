use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Task {
    priority: usize,
    description: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut task_queue = BinaryHeap::new();

    task_queue.push(Task { priority: 3, description: String::from("Clean area") });
    task_queue.push(Task { priority: 5, description: String::from("Charge battery") });
    task_queue.push(Task { priority: 1, description: String::from("Move to location") });

    while let Some(task) = task_queue.pop() {
        println!("Executing task: {} (Priority: {})", task.description, task.priority);
    }
}
