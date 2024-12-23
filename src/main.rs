use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("input un nombre");

    let lenombre: i32 = rand::thread_rng().gen_range(1..101);

    
    loop {
        let mut entre_utilisateur = String::new();

        println!("test entre : {}",entre_utilisateur);
        io::stdin()
            .read_line(&mut entre_utilisateur)
            .expect("echec de l'input");

        let entre_utilisateur: i32 = match entre_utilisateur.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => {
                println!("Il faut un nombre positif");
                continue;
            }
        };

        match entre_utilisateur.cmp(&lenombre) {
            Ordering::Equal => {
                println!("c bon");
                break;
            }
            Ordering::Greater => println!("c moins "),
            Ordering::Less => println!("c plus"),
        }
    }
}
