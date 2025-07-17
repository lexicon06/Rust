fn main() {
    let valor:i32 = 10;
    println!("El valor de la variable es: {}", valor);

    let valor = 20; // shadowing

    println!("El valor nuevo es: {}", valor);

    let valor = false;

    println!("El valor es: {}", valor);
}
