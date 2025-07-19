use std::io;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    init();
}

fn init() {
    println!("Demo program #2 🦀");
    sleep(Duration::from_secs(3));
    let user_name = register();
    println!("Thank you, {}!", user_name);
    menu();
}

fn register() -> String {
    println!("Please enter your nickname:");
    let mut nickname = String::new();
    io::stdin()
        .read_line(&mut nickname)
        .expect("Problem reading nickname");
    nickname.trim().to_string()
}

fn menu() {
    loop {
        println!("\nPlease choose your destiny:");
        println!("1. How to define Arrays in Rust");
        println!("2. How to define Tuples in Rust");
        println!("3. How to define Vectors in Rust");
        println!("4. How to add data in Arrays in Rust");
        println!("5. How to add data in Tuples in Rust");
        println!("6. How to add data in Vectors in Rust");
        println!("0. Exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        let option = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 0 and 6.");
                continue;
            }
        };

        if option == 0 {
            println!("Goodbye! 🚀");
            break;
        }

        menu_handler(option);
    }
}

fn menu_handler(option: i32) {
    match option {
        1 => arrays_definition(),
        2 => tuples_definition(),
        3 => vectors_definition(),
        4 => arrays_data(),
        5 => tuples_data(),
        6 => vectors_data(),
        _ => println!("Invalid option. Please choose between 0 and 6."),
    }
}

fn wait(){

    println!("\nHit Enter Key to continue.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
}

fn arrays_definition() {
    println!("\n📚 Arrays in Rust:");
    println!("let num_array = [1, 2, 3, 4, 5];");
    println!("let abc_array = ['a', 'b', 'c'];");
    println!("Use .len() to get length and array[index] to access elements.");
    wait();
}

fn tuples_definition() {
    println!("\n📚 Tuples in Rust:");
    println!("let person = (\"Alice\", 30, true);");
    println!("Mix types and access with .0, .1, .2.");
    println!("Destructure with: let (name, age, active) = person;");
    wait();
}

fn vectors_definition() {
    println!("\n📚 Vectors in Rust:");
    println!("let numbers = vec![1, 2, 3];");
    println!("Use .push() to add data and [index] to access.");
    println!("Get length with .len()");
    wait();
}

fn arrays_data() {
    println!("\n🔧 Arrays: fixed size, but mutable values.");
    println!("let mut scores = [10, 20, 30];");
    println!("scores[1] = 99;");
    println!("Loop like:");
    println!("for num in scores.iter() {{ println!(\"{{}}\", num); }}");
    wait();
}

fn tuples_data() {
    println!("\n🔧 Tuples: mutable and destructurable.");
    println!("let mut info = (\"Bob\", 42, true);");
    println!("info.0 = \"Alice\";");
    println!("let (name, age, status) = info;");
    wait();
}

fn vectors_data() {
    println!("\n🔧 Vectors: growable and flexible.");
    println!("let mut notes = vec![7, 8, 9];");
    println!("notes.push(10);");
    println!("Loop like:");
    println!("for val in &notes {{ println!(\"{{}}\", val); }}");
    wait();
}
