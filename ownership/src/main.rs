struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    // Ownership
    // Cada valor en Rust tiene su propio ownership.
    // Solo puede existir un ownership a la vez.
    // Si un ownership sale del alcance, el valor se descartará.

    let rectangle = Rectangle { width: 10, height: 20 };

    let new_rectangle = rectangle; // <-- al hacer esto el ownership pasa a ser de new_rectangle
    // Los argumentos son pasados mediante prestamos -> default
    // Los argumentos sean pasados por referencias asi podemos recuperar el ownership
    let result = area(&new_rectangle);

    println!("Area is: {}", result);
    println!("Width: {}, Height: {}", new_rectangle.width, new_rectangle.height);

    
}