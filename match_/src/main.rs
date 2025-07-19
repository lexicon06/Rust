use std::io;

fn main() {
    let mut user_input = String::new();

    println!("Ingresa un numero del 1 a 7");

    io::stdin().read_line(&mut user_input).expect("There was a problem reading your input");

    user_input = user_input.trim().to_string();

    let numero: i32 = user_input.parse().unwrap();


    match numero {
        1 => println!("Literal match found -> 1"),
        2 => println!("Literal match found -> 2"),
        3 => println!("Literal match found -> 3!!"),
        4 | 5 | 6 => println!("Literal found 4 ~ 6"),
        _ => println!("Literal Match found: {numero}")
    }


    let fco;

    let match_output = match numero {
        1 => "Match Output -> 1",
        2 => "Match Output -> 2",
        3 => "Match Output -> 3",
        4 => "Match Output -> 4",
        5 | 6 | 7 => "Match Output > 5 ~ 7",
        _ => {
            fco = format!("Match Output > 7 = {}", numero);
            &fco
        }
        
    };


    println!("match output fn: {match_output}");
}
