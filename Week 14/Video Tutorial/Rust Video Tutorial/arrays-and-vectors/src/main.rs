fn main() {
    let working_days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    let working_days_num = [0; 5];

    println!("{}", working_days[0]);

    let nephews_age = vec![14, 9, 0];
    println!("Newphews age: {:?}", nephews_age);

    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    let mut names = Vec::new();

    names.push("Faiz");
    names.push("Will");
    names.push("Isaac");

    println!("Names: {:?}", names);

    names.pop();
    println!("Names: {:?}", names);

    let mut fruit = vec!["Apple", "Melon", "Orange"];
    let orange = fruit[2];
    println!("Fruits: {:?}, Orange = {}", fruit, orange);
    fruit[0] = "Strawberry";
    println!("Fruits: {:?}, Orange = {}", fruit, orange);

}
