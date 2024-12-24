pub mod jeu;
fn main() {
    let unjeu = jeu::JeuDeviner::
        new(10, 100, jeu::Difficulte::Facile);
    
    println!("struct : {:#?}", &unjeu);
    unjeu.jouer();
}
