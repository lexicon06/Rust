fn main() {
    saludar_usuarios();
    println!("{}", suma(5,5));
}


fn saludar_usuarios(){
    println!("Hola usuarios");
}

fn suma(num1:i32, num2:i32) -> i32{
    let resultado = num1 + num2;
    resultado
}