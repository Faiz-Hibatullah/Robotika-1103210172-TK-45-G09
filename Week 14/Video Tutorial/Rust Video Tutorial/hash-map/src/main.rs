fn main() {
    use std::collections::HashMap;
    let mut items: HashMap<String, String> = HashMap::new();

    items.insert(String::from("One"), String::from("Book"));
    items.insert(String::from("Two"), String::from("Keyboard"));
    items.insert(String::from("Three"), String::from("Glasses"));

    let keyboard = items.get("Two");
    println!("{:?}", keyboard);

    items.remove("Three");

    println!("{:?}", items.get("Three"));

}
