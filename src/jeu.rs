
use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
pub enum Difficulte {
    Facile,
    Moyen,
    Difficile,
}

#[derive(Debug)]
pub struct JeuDeviner {
    essais: i32,
    nb_alea: i32,
    reponses: Vec<i32>
}

impl JeuDeviner {
    pub fn new(mut essais: i32, mut marge_max: i32, niveau: Difficulte) -> Result<Self, io::Error> {
        if let Difficulte::Facile = niveau {
            essais *= 2;
            marge_max /= 2;
        } else if let Difficulte::Difficile = niveau {
            essais /= 2;
            marge_max *= 2;
        }
        let alea = Self::calcul_alea(marge_max);
        if essais <= 1 || marge_max <= 1{
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Ne pas mettre de valeurs négatives"))
        }
        else {
            Ok(Self {
                essais,
                nb_alea: alea,
                reponses: Vec::new()
            })
        }
    }

    fn calcul_alea(marge_max: i32) -> i32 {
        rand::thread_rng().gen_range(1..=marge_max)
    }

    pub fn jouer(&mut self) -> bool {
        let mut gagner = false;
        let mut i = self.essais;
        'essais: loop {
            if i <= 0 {
                println!(
                    "A court d'essais, perdu! Le nombre était : {}",
                    self.nb_alea
                );
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
            self.reponses.push(entre_utilisateur);
            i -= 1;
        }
        gagner
    }

    pub fn show_answer(&self) {
        println!("Voici les différents input :");
        for (i,ans) in self.reponses.iter().enumerate() {
            println!("{} : {}",i+1, ans);
        }
    }
}
