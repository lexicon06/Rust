use std::io;



struct Room{
    name: String,
    external_ip: String,
    local_ip: String,
    port: i32,
    topic: String,
}

impl Room{
    fn hello(&self){
        println!("Welcome to {}", self.name);
    }
}

fn main(){
    println!("Welcome to demo cli");
    let mut user_name = String::new();

    println!("Please enter your name:");

    io::stdin().read_line(&mut user_name).expect("There was a problem with your request");

    user_name = user_name.trim().to_string();

    println!("That's it {user_name}");


    /*
    let mut num1: i32 = 0;
    let mut num2: i32 = 0;

    */

    let mut num: [i32; 2] = [0, 0];

    (num[0], num[1]) = tupl();

    let mut a:i32; // No need to be mutable if x1 when tuple, if we tr to modify later we need mut
    let mut b:i32;

    (a,b) = tupl();

    match (true,false){
        (true, false) => println!("Ok"),
        (false, false) => println!("Err"),
        _ => println!("Something else")
    }

    println!("a: {a}, b: {b}");

    (b,a) = tupl(); //if we want to reverse we need our variable to be mut

    println!("a: {a}, b: {b}");

    let mut pos: Vec<f32> = Vec::new();
/*
    pos.push(0.25564);
    pos.push(5.60033);
    pos.push(0.04635);
    
    */


    // pos[0] = 0.32; // we cannot add if not defined early

    pos.push(0.234);

    pos.push(0.345);

    pos.push(0.234);

    pos.push(0.345);

    pos.push(0.234);

    pos.push(0.345);

    pos.pop();


    println!("Vector {:?}", pos);


    //loop

    for val in 0..pos.len() {
        println!("{}", pos[val]);
    }

    for i in 0..10 {
        println!("{i}");
    }

    for i in 100..200 {
        println!("{i}");
    }


    println!("{:?}, {0}, {1}", num[0], num[1]);


    let room_name: String;
    let room_external_ip: String;
    let room_local_ip: String;
    let room_port: i32;
    let room_topic: String;


    (room_name, room_external_ip, room_local_ip, room_port, room_topic) = constructor();
    let room = Room{
        name: room_name,
        external_ip: room_external_ip,
        local_ip: room_local_ip,
        port: room_port,
        topic: room_topic,
    };


    println!(
    "ROOM OK: name: {}, ex ip: {}, loc ip: {}, topic: {}, port: {}",
    room.name, room.external_ip, room.local_ip, room.topic, room.port
);


    room.hello();




}


fn constructor() -> (String, String, String, i32, String){

    let mut name: String = String::new();
    let mut ext_ip: String = String::new();
    let mut loc_ip: String = String::new();
    let mut port: String = String::default();
    let mut topic: String = String::new();
    let msg: &str = "Something went wrong, we couldn't get your input";

    println!("Please enter room name:");

    io::stdin().read_line(& mut name).expect(msg);

    name = name.trim().to_string();

    println!("Please enter external ip:");

    io::stdin().read_line(&mut ext_ip).expect(msg);

    ext_ip = ext_ip.trim().to_string();

    println!("Please enter local ip:");

    io::stdin().read_line(&mut loc_ip).expect(msg);

    loc_ip = loc_ip.trim().to_string();

    println!("Please enter port:");

    io::stdin().read_line(&mut port).expect(msg);

    println!("Please enter topic:");

    io::stdin().read_line(&mut topic).expect(msg);

    topic = topic.trim().to_string();


    (name, ext_ip, loc_ip, port.trim().parse().unwrap_or(0), topic)


}

fn tupl() -> (i32, i32){
    //on this we are returning tuple


    (1,2)
}