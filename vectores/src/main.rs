fn main() {
    //let vector = vec![1,5,7];

    let mut vector = vec![1,3,5,6];

    vector.push(8);

    vector.insert(1,2);

    vector.insert(2,3);

    vector.remove(vector.len()-1);
    vector.remove(vector.len()-1);
    vector.remove(vector.len()-1);
    vector.remove(vector.len()-1);

    vector.remove(0);

    vector[1]=99;

    vector.pop();

    vector.push(4);
    vector.push(8);

    println!("vec is {:?}", vector);


    let mut new_vector: Vec<i32> = Vec::new();

    new_vector.push(1);

    println!("Is {:?}", new_vector);
}
