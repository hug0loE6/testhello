mod jeu;

pub use crate::jeu::{JeuDeviner, Difficulte};
fn main() {
    let unjeu = JeuDeviner::new(10, 100, Difficulte::Facile);
    println!("struct : {:#?}", &unjeu);
    unjeu.jouer();
}
