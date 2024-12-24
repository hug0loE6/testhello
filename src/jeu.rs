use rand::Rng;
use std::cmp::Ordering;
use std::io;
#[derive(Debug)]
pub struct JeuDeviner {
    essais: i32,
    nb_alea: i32,
}

impl JeuDeviner {
    pub fn new(essais: i32, marge_max: i32) -> Self {
        let alea = Self::calcul_alea(marge_max);
        Self {
            essais,
            nb_alea: alea,
        }
    }

    fn calcul_alea(marge_max: i32) -> i32 {
        rand::thread_rng().gen_range(1..=marge_max)
    }

    pub fn jouer(&self) -> bool {
        let mut gagner = false;
        let mut i = self.essais;
        'essais: loop {
            if i <= 0 {
                println!("A court d'essais, perdu! Le nombre était : {}", self.nb_alea);
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

            match entre_utilisateur.cmp(&self.nb_alea) {
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
}
