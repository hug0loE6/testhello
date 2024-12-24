mod jeu;

pub use crate::jeu::{JeuDeviner, Difficulte};
fn main() {
    let mut unjeu = JeuDeviner::new(10, 100, Difficulte::Facile);
    println!("struct : {:#?}", &unjeu);
    unjeu.jouer();
    unjeu.show_answer();
}
