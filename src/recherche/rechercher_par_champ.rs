use crate::livres::bibliotheque::Bibliotheque;
use crate::livres::livre::Livre;
use crate::livres::enum_champ::Champ;


pub fn recherche_par_champ(bibliotheque: &Bibliotheque, champ: Champ, recherche: &str) -> Vec<Livre> {
    let mut resultats: Vec<Livre> = Vec::new();
    let recherche = recherche.to_lowercase();
    for livre in &bibliotheque.livres {
        let trouve = match champ {
            Champ::Titre => livre.titre.to_lowercase().contains(&recherche),
            Champ::Auteur => livre.auteur.to_lowercase().contains(&recherche),
            Champ::Genre => livre.genre.to_lowercase().contains(&recherche),
            _ => false,
        };
        
        if trouve {
            resultats.push(livre.clone());
        }
    }

    resultats
}




