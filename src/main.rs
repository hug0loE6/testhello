use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let lenombre = gen_alea(100);

    guess(lenombre, 10);
}

fn gen_alea(rangemax: i32) -> i32 {
    rand::thread_rng().gen_range(1..=rangemax)
}

fn guess(deviner: i32, nbessais: u8) -> bool {
    let mut gagner = false;
    let mut i = nbessais;
    'essais: loop {
        if i <= 0 {
            println!("A court d'essais, perdu! Le nombre était : {}", deviner);
            break 'essais;
        }
        println!("essais restants : {}", i);
        println!("input un nombre : ");
        let mut entre_utilisateur = String::new();
        
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

        match entre_utilisateur.cmp(&deviner) {
            Ordering::Equal => {
                println!("c bon");
                gagner = true;
                break 'essais;
            }
            Ordering::Greater => println!("c moins "),
            Ordering::Less => println!("c plus"),
        }
        i -= 1;    
    }
    gagner
}
