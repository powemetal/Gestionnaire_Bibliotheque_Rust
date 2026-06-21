use crate::affichage::affichage_menu_recherche_champ::menu_recherche_par_champs;
use crate::affichage::affichage_tableau::afficher_tableau;

// use crate::livres::livre::Livre;
use crate::livres::champ::Champ;
use crate::livres::bibliotheque::Bibliotheque;
use crate::livres::statut::Statut;
use crate::livres::livre::Livre;

use crate::recherche::rechercher_par_champ::recherche_par_champ;
use crate::utils::input_valeur::input_valeur;
use crate::utils::validation::valider_choix;


pub fn rechercher(bibliotheque: &Bibliotheque) {
    let bibliotheque = &bibliotheque;
    loop {
        menu_recherche_par_champs();
        let choix_utilisateur = valider_choix(6);
        match choix_utilisateur {
            Some(1) => {
                let recherche = input_valeur("Entrez le titre a rechercher: ");
                let resultat = recherche_par_champ(&bibliotheque, Champ::Titre, &recherche);
                afficher_tableau(&resultat, &format!("\nResultats trouves avec titre: \"{}\"", &recherche));
            },

            Some(2) => {
                let recherche = input_valeur("Entrez l'auteur a rechercher: ");
                let resultat = recherche_par_champ(&bibliotheque, Champ::Auteur, &recherche);
                afficher_tableau(&resultat, &format!("Resultats trouves avec auteur \"{}\"", &recherche));
            },
            Some(3) => {
                let recherche = input_valeur("Entrez le genre a rechercher: ");
                let resultat = recherche_par_champ(&bibliotheque, Champ::Genre, &recherche);
                afficher_tableau(&resultat, &format!("Resultats trouves avec genre \"{}\"", &recherche));
            },

            Some(4) => {
                let resultat = bibliotheque.livres.iter().filter(|l| l.statut == Statut::Disponible).cloned().collect::<Vec<Livre>>();
                afficher_tableau(&resultat, &"Lire disponible:");
            },
            Some(5) => {
                let resultat = bibliotheque.livres.iter().filter(|l| l.statut == Statut::Emprunte).cloned().collect::<Vec<Livre>>();
                afficher_tableau(&resultat, &"Livres emprunté:");
            },
            Some(6) => {
                break
            },
            _ => {},
        }
    }
}