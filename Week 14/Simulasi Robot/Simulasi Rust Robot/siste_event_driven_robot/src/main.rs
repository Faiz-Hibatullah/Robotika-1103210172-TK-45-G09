fn main() {
    let mut events = vec!["obstacle_detected", "goal_changed"];

    for event in &events {
        match *event {
            "obstacle_detected" => println!("Adjusting path..."),
            "goal_changed" => println!("Recalculating route..."),
            _ => println!("No action needed."),
        }
    }
}
