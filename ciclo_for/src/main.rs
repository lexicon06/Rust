fn main() {
    let numeros: [i32; 5] = [1, 2, 3, 4, 5];

    for numero in numeros.iter() {
        println!("{}", numero);
    }

    /*
    for numero in 1..101 { //ranges btween 1 / 100
        println!("{numero}");
    }
     */

    for numero in 1..101 {
        if numero % 3 == 0 && numero % 5 == 0 {
            println!("Fizz buzz");
        } else if numero % 3 == 0 {
            println!("Fizz");
        } else if numero % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{numero}");
        }
    }

    // Fizz Buzz
}
