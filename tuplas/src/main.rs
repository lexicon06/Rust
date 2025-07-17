fn main() {
    let tupla = (false, true, 1, "Yes");
    let mut tupla2: (i32, bool, f32) = (10, false, 1.5);

    println!("El valor es {:?} & {:?}", tupla, tupla2);

    let primer_elemento_t1 = tupla.0;
    let ultimo_elemento_t1 = tupla.3;


    tupla2.1 = true;

    println!("{}", tupla2.1);

    println!("Primer {}, Segundo {}", primer_elemento_t1, ultimo_elemento_t1);
}
