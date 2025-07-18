fn main() {
    //let numeros = [1,2,3,4,5,6,7,8,9,10];
    let numeros: [i32;5] = [1,2,3,4,5];
    let valores = [1; 10];

    println!("{}", numeros[3]);

    println!("{:?}", numeros);// to know what contains on the array

    println!("{:?}", valores);

    let primer_elemento = numeros[0];
    let ultimo_elemento = numeros[numeros.len()-1];

    println!("El 1er elemento es {}", primer_elemento);
    println!("El ultimo elemento es {}", ultimo_elemento);
}
