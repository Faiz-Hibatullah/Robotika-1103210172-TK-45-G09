use std::io;

fn main() {
    let mut position = (0, 0);

    loop {
        println!("Robot is at position: {:?}", position);
        println!("Enter movement (up, down, left, right) or 'exit':");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();

        match command {
            "up" => position.1 += 1,
            "down" => position.1 -= 1,
            "left" => position.0 -= 1,
            "right" => position.0 += 1,
            "exit" => break,
            _ => println!("Invalid command!"),
        }
    }
}
