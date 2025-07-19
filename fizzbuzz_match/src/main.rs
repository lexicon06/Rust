fn main() {
    for numero in 1..31 {
        match (numero % 3, numero % 5) {// so this evaluates and later i can check whether is true or not
            (0, 0) => println!("Fizz Buzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", numero),
        }
    }
}
