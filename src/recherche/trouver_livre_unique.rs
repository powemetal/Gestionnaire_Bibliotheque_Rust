use crate::livres::bibliotheque::Bibliotheque;
use crate::livres::livre::Livre;

use crate::affichage::affichage_tableau::afficher_tableau;
use crate::utils::validation::valider_choix;


pub fn trouver_livre_unique(bibliotheque: &Bibliotheque, titre: &str) -> Option<Livre> {
    let mut resultats = Vec::new();
    for livre in &bibliotheque.livres {
        if livre.titre.to_lowercase().contains(titre.to_lowercase().trim_end()) {
            resultats.push(livre.clone());
        }
    }
    match resultats.len() {
        0 => None,
        1 => Some(resultats[0].clone()),
        _ => {
            afficher_tableau(&resultats, "Livres trouvés");
            let choix = match valider_choix(resultats.len() as i32) {
                Some(c) => c-1,   
                None => return None,
            };
            let choix = choix as usize;
            Some(resultats[choix].clone())
        }
    }
}



