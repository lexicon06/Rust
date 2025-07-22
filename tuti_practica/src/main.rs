use std::io;

use std::thread::sleep;
use std::time::Duration;


struct Restaurant {
    wallet: f32,
}

impl Restaurant {
    fn new() -> Self {
        Restaurant { wallet: 0.0 }
    }

    fn intro(&self) {
        println!("=============================");
        println!("=============================");
        println!("Bienvenido a la polleria toti 🐔");
        println!("Para continuar selecciona del menu");
        println!("=============================");
        println!("=============================");
    }

    fn display_menu(&self) {
        println!("\n--- MENÚ ---");
        println!("1. 🍔 InkaBurger - $6.25");
        println!("2. 🍗 Pollito broaster - $12.25");
        println!("3. 🍟 Salchipapa - $5.50");
        println!("4. 🐥 Pollito a la brasa - $15.80");
        println!("5. 🥤 Inka Cola - $3.25");
        println!("6. 💰 Ver total gastado");
        println!("7. 🛎️  Realizar pedido");
        println!("8. 🛫 Salir");
        println!("\nIngresa tu pedido:");
    }

    fn get_user_input(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let selection = input.trim().parse::<i32>()?;
        Ok(selection)
    }

    fn get_item_details(&self, option: i32) -> Option<(String, f32)> {
        match option {
            1 => Some(("InkaBurger 🍔".to_string(), 6.25)),
            2 => Some(("Pollito broaster 🍗".to_string(), 12.25)),
            3 => Some(("Salchipapa 🍟".to_string(), 5.50)),
            4 => Some(("Pollito a la brasa 🐥".to_string(), 15.80)),
            5 => Some(("Inka Cola 🥤".to_string(), 3.25)),
            _ => None,
        }
    }

    fn add_to_order(&mut self, item_name: String, cost: f32) {
        self.wallet += cost;
        println!("✅ Has pedido: {} - ${:.2}", item_name, cost);
        println!("💰 Total gastado hasta ahora: ${:.2}", self.wallet);
        sleep(Duration::from_secs(2));
    }

    fn show_total(&self) {
        println!("💰 Total gastado: ${:.2}", self.wallet);
        sleep(Duration::from_secs(2));
    }

    fn run(&mut self) {
        self.intro();

        loop {
            self.display_menu();

            match self.get_user_input() {
                Ok(selection) => {
                    match selection {
                        1..=5 => {
                            if let Some((item_name, cost)) = self.get_item_details(selection) {
                                self.add_to_order(item_name, cost);
                            }
                        }
                        6 => {
                            self.show_total();
                        }
                        7 => {


                            if self.wallet == 0.00 {
                                println!("📯 No atendemos boludos, bye 🏃");
                                break;
                            }

                            println!("¡Gracias por realizar tu pedido: Iremos cuanto antes 🏍️📦");
                            println!("Total final: ${:.2}", self.wallet);
                            sleep(Duration::from_secs(10));
                            break;
                            
                        }

                        8 => {
                            println!("¡Gracias por tu visita a Pollería Toti! 🐔");
                            println!("Total final: ${:.2}", self.wallet);
                            break;
                        }
                        _ => {
                            println!("❌ Opción inválida. Por favor selecciona del 1 al 7.");
                        }
                    }
                }
                Err(_) => {
                    println!("❌ Entrada inválida. Por favor ingresa un número.");
                }
            }

            println!("\n{}\n", "-".repeat(40));
        }
    }
}

fn main() {
    let mut restaurant = Restaurant::new();
    restaurant.run();
}