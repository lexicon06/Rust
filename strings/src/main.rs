fn main() {
    // (stack)str -> Es una cadena inmutable
    // (heap)String -> String es una cadena mutable | Agregar o Quitar
    let variable_str = "Hola, soy un tipo str (no muto)";//goes to stack
    //let variable_string = String::new(); // goes to heap
    //let variable_string = String::from("hola soy un string");
    let mut variable_string = String::from("Soy un String");

    variable_string.push(',');
    variable_string.push_str(" Hola");



    println!("El str es : {variable_str}");
    println!("El String es : {variable_string}");


    let str_to_string = "String demo conversion".to_string();


    println!("{str_to_string}");


    let string_to_str = str_to_string.as_str();

    println!("{string_to_str}");

    let is_equal: bool = string_to_str == str_to_string;

    println!("Is {string_to_str} equal to {str_to_string} ? {is_equal}");

}
