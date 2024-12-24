pub mod jeu;
fn main() {
    let unjeu = jeu::JeuDeviner::new(10, 100);
    println!("struct : {:#?}", &unjeu);
    unjeu.jouer();
}

