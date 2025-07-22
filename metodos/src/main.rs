struct User {
    name: String,
    password: String,
}

impl User {
    fn saluda(&mut self){
        println!("Hola, soy el usuario {}", self.name);
    }

    fn change_password(&mut self, new_password: String){
        self.password = new_password;
        println!("El password fue cambiado exitosamente");
        println!("DEBUG: {}", self.password);
    }
}


fn main(){

    println!("CLI AGAIN!");

    let mut usuario = User {
        name: String::from("Pablo"),
        password: String::from("DjangoJoestar"),
    };



    usuario.saluda();

    usuario.change_password(String::from("1231234"));


}