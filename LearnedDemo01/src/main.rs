use std::io;
use std::thread::sleep;
use std::time::Duration;


fn main() {
    println!("Demo has initiated correctly");
    sleep(Duration::from_secs(2));
    let user_name = welcome();
    menu(&user_name);
}

fn welcome() -> String {
    println!("Welcome to the exercise\nPlease enter your username:");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Sorry, we couldn't read your username");

    println!("Nice to see you again: {}", username);

    username.trim().to_string()
}

fn menu(user_name: &str) {
    let mut input = String::new();

    
    println!("What would you like to do today?");
    println!("1. Show an array of numbers");
    println!("2. Show a tuple of data");
    println!("3. Push data to an array (same data type)");
    println!("4. Exit");

    io::stdin().read_line(&mut input).expect("Something went wrong reading your option.");

    let option: i8 = match input.trim().parse::<i8>() {
        Ok(data) if data >= 1 && data <= 4 => data,
        _ => {
            println!("\n🚨 Por favor {}, ingresa un número entre el 1 y el 4.\n", user_name);
            sleep(Duration::from_secs(1));
            return menu(user_name);
        }
    };

    println!("You have selected {}", option);
}