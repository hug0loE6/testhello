mod jeu;

pub use crate::jeu::{JeuDeviner, Difficulte};
fn main() {

    let unjeu = JeuDeviner::new(-10, 100, Difficulte::Facile);
    let mut unjeu = match unjeu {
        Ok(jeu) => jeu,
        Err(msg) => panic!("{}",msg)
    };
    println!("struct : {:#?}", &unjeu);
    unjeu.jouer();
    unjeu.show_answer();
}
