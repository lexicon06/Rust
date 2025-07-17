fn main() {

    //same comment type
    /* as js */
    let numero_uno = 15;
    let mut random_value = 3;
    println!("Ahora soy {}", random_value);
    random_value = 10;
    println!("Ahora soy {}", random_value);
    let numero_dos:i32 = 10;
    let resultado_final = numero_uno + numero_dos;
    println!("El resultado de {} + {} es: {}", numero_uno, numero_dos, resultado_final);

    //una constante en rust no es lo mismo que una variable inmutable usamos const o static, al ser constante debemos especificar el tipo de dato

    const VALOR:i32 = 3;

    println!("Oh no, the constant value is {}", VALOR);


}
