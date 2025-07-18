use std::io;

fn main() {
    let mut color = String::new();
    const SEMAFORO: [&str; 3] = ["verde","amarillo","rojo"];

    println!("Please enter the color");

    io::stdin().read_line(&mut color).expect("Error, we couldn't get the color");

    let color = color.trim();

    if color == SEMAFORO[0] {
        println!("Puede continuar");
    }else if color== SEMAFORO[1]{
        println!("Precaucion");
    }else{
        println!("Alto total, el semaforo esta en rojo");
    }
}
