fn main() {
    let mensaje:&str = "Hola soy una variable en el bloque main";

    {
        println!("Hola desde un segundo bloque");

        println!("{}", mensaje);

        let mensaje2 = "Hola soy una variable en un bloque anidado";

        println!("{}", mensaje2);
    }

    //println!("{}", mensaje2); won't work.. it will work only on the scope which was defined

    println!("{}", mensaje);

    let resultado = {
        println!("Hola nos encontramos en un bloque anidado");

        let variable: i32 = 200;

        variable // <-- pay attention, here is with no;
    };

    println!("El valor de nuestra variable es {}", resultado);


    //otro



    let calificacion: i8 = 10;

    let msg = {

    if calificacion == 10 {
        String::from("Felicidades, has obtenido la calificación máxima")
    }else{
        String::from("Debes estudiar más, suerte en la próxima")
    }

};

    println!("{}", msg);

}
