use std::io;


fn main(){

    let mut user_name = String::new();
    let mut user_age = String::new();

    println!("Welcome to my program\nPlease enter your name:");

    io::stdin().read_line(&mut user_name).expect("there was an error with the reading of the line");

    let user_name = user_name.trim(); // name collected ok


    println!("Please enter your age mate");

    io::stdin().read_line(&mut user_age).expect("There was a problem reading your age");

    //cleaning + parsing 

    let user_age = user_age.trim();

    let age = match user_age.parse(){
        Ok(age)=>age,
        Err(_)=>0,
    };



    println!("succes {}-name,{}-yrsold", user_name, age);




}