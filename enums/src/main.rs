fn main() {
    
    enum Response {
        Success,
        Error(u32, String),
    }

    {
        let _ok = Response::Success;
    }

    let mut respuesta = Response::Error(403, String::new());

    respuesta = Response::Error(501, format!("El error es critico"));

    match respuesta {
        Response::Success => println!("La petición se completó correctamente"),
        Response::Error(403, _) => {
            println!("La petición no ha podido completarse: Forbidden")
        }
        Response::Error(_, msg) => println!("Error: {}", msg),
    }
}
