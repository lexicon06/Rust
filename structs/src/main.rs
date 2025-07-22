use std::io;

struct User{
    name: String,
    password: String,
}


fn main(){
    println!("Hello, welcome to my CLI");
    let user: User = register();

    println!("Your password has been received {}", user.password);

}

fn register() -> User{
    let mut name: String = String::new();
    let mut password: String = String::new();
    let i_msg: &str = "Sorry, we couldn't get what you just typed";

    println!("Please enter your name:");

    io::stdin().read_line(&mut name).expect(i_msg);

    name = name.trim().to_string();

    println!("Please enter your password:");

    io::stdin().read_line(&mut password).expect(i_msg);

    password = password.trim().to_string();

    let user: User = create_user(name, password);

    println!("Welcome {}!", user.name);

    user

}


fn create_user(name: String, password: String) -> User{
    User { name: name, password: password }
}