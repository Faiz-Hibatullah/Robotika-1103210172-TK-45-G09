struct Student {
    name: String,
    level: u8,
    remote: bool
}

struct Grade(char, char, char, f32);

fn main() {
    println!("Hello, {}", "Faiz");

    let mut age = 21;
    let birth_year = 2003;

    println!("I am {} years old", age);

    let birth_year = birth_year-1;

    age = 20;

    println!("I am now {} years old", age);
    println!("I was born in {}", birth_year );

    let nephew_age: u32 = 14;
    println!("My nephew is {} years old", nephew_age);

    let _float: f32 = 4.0;

    println!("1 + 2  = {}", 1*2);

    let is_bigger_num = 2 < 4;
    println!("Is 2 < 4: {}", is_bigger_num);

    let first_char: char = 'F';
    let last_char: char = 'Z';

    let second_char = 'A';

    let my_name = "Faiz";

    println!("{} is the first character, {} is the last character, {} is the second character of my name {}", first_char, last_char, second_char, my_name);

    let my_cat = ("bong bong", 5, true);

    println!("My cat's name is {}, he is {} years old, is he alive? {}", my_cat.0, my_cat.1, my_cat.2);

    let student_1 = Student{
        name: String::from("Faiz"),
        remote: true,
        level: 5
    };

    let grades = Grade('A', 'A', 'B', 3.5);

    println!("{}, is a level {} programmer. Does he work remotely {}",
        student_1.name, student_1.level, student_1.remote);

    println!("{} {} {}, GPA = {}", grades.0, grades.1, grades.2, grades.3)
    
}
