use crate::livres::livre::{AfficherStatut, Livre};


pub fn afficher_tableau(livres: &[Livre], titre: &str) {
    println!("\n===========================================================================================================");
    println!("{:^92}", titre);
    println!(" __________________________________________________________________________________________________________");
    println!("| {:^5.5}| {:<30} | {:<20} | {:<6} | {:<6} | {:<10} | {:<10} |", "#", "Titre", "Auteur", "Année", "Pages", "Genre", "Statut");
    println!("|----------------------------------------------------------------------------------------------------------|");
    // let mut compteur = 0;
    for (i, livre) in livres.iter().enumerate() {
        // compteur += 1;
        println!(
            "| {:^5.5}| {:<30.30} | {:<20.20} | {:<6} | {:<6} | {:<10.10} | {:19.19} |",
            i+1, livre.titre, livre.auteur, livre.annee, livre.pages, livre.genre, livre.statut.afficher());
    }
    println!(" ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯");
    println!();
}