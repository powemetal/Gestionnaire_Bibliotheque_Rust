use crate::affichage::affichage::afficher_message_jaune;

use std::io::{stdin, stdout, Write};

pub fn valider_choix(limit: i32) -> Option<i32> {
    let mut choix = String::new();
    print!("Choix (1-{})> ", limit);
    stdout().flush().unwrap();

    stdin().read_line(&mut choix).expect("Erreur lors de la lecture de la ligne.");

    let choix: i32 = match choix.trim().parse(){
        Ok(valeur) => valeur,
        Err(erreur) => {
            afficher_message_jaune(&format!("Veuillez entrer un nombre valide: {}\n", erreur));
            return None;
        },
    };
    
    if choix < 1 || choix > limit {
        afficher_message_jaune(&format!("Veuillez entrer un nombre entre 1 et {}\n", limit));
        None
    } else {
        Some(choix)
    }
    
}