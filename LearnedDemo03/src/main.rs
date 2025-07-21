use std::io;

struct User {
    name: String,
    address: String,
    phone: String,
    age: i32,
    zip_code: i32,
}

fn main(){
    init();
}

impl User{
    fn greet(&self){
        println!("Thank you for your registration");
        println!("This is what we collected so far");
        println!("Name: {0}, Age: {4}, Address: {2}, Phone: {3}, ZipCode: {1}",
        self.name, self.zip_code, self.address, self.phone, self.age);//Just to mess i have added a random order.
    }
}


fn init(){
    println!("Welcome to my CLI, enter your details to continue");
    register().greet();
}


fn register() -> User{
    let mut name = String::new();
    let mut address = String::new();
    let mut phone = String::new();
    let mut age = String::new();
    let mut zip_code = String::new();
    let i_warning_msg: &str = "Sorry, we couldn't read your input";

    println!("Please enter your name:");
    io::stdin().read_line(&mut name).expect(i_warning_msg);

    println!("Please enter your address:");
    io::stdin().read_line(&mut address).expect(i_warning_msg);

    println!("Please enter your phone:");
    io::stdin().read_line(&mut phone).expect(i_warning_msg);

    println!("Please enter your age:");
    io::stdin().read_line(&mut age).expect(i_warning_msg);

    println!("Please enter your zip_code:");
    io::stdin().read_line(&mut zip_code).expect(i_warning_msg);

    let mut user = User {
        name: name.trim().to_string(),
        address: address.trim().to_string(),
        phone: phone.trim().to_string(),
        age: age.trim().parse().unwrap_or(0),
        zip_code: zip_code.trim().parse().unwrap_or(0),
    };

    println!("User registered: {} | {} | {}", user.name, user.address, user.phone);

    user.name = "Ignacio".to_string();

    user
}