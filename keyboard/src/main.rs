use std::io;

fn main() {
    println!("Enter your username:");
    let mut user_name = String::new(); // new string = ""
    let mut user_age = String::new();
    //Result Success or Failed
    let _ = io::stdin().read_line(&mut user_name).expect("Something went wrong, can't catch your name"); // we use ref ~ puntero, w "mut" we tell is able to write as well
    let user_name = user_name.trim();// delete odd characters w shadowing
    println!("Enter your age:");
    let _ = io::stdin().read_line(&mut user_age).expect("Something went wrong, can't catch your age");
    let user_age = user_age.trim();
    //success or failed result:
    let age:i32 = user_age.parse().unwrap();//we parse our number to real int32

    println!("Welcome back, {}, happy {}yo!", user_name, age);

}
