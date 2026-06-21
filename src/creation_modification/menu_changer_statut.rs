use crate::affichage::affichage_couleurs_messages::afficher_message_jaune;
use crate::affichage::affichage_tableau::afficher_tableau;
use crate::affichage::affichage_menu_changer_statut::affichage_menu_changer_statut;

use crate::livres::bibliotheque::Bibliotheque;

use crate::recherche::trouver_livre_unique::trouver_livre_unique;

use crate::utils::input_valeur::input_valeur;
use crate::creation_modification::changer_statut_livre::changer_statut_livre;


pub fn menu_changer_statut(bibliotheque: &mut Bibliotheque) {
    loop {
        affichage_menu_changer_statut();
        let recherche = input_valeur("Titre (Vide pour afficher toute la liste) > ");

        let livre = match trouver_livre_unique(bibliotheque, &recherche){
            Some(l) =>l,
            None => {
                println!("Aucun livre trouvé\n");
                break
            },
        };


        let index = match bibliotheque.livres.iter().position(|l| l == &livre) {
            Some(i) => i,
            None => {
                afficher_message_jaune("Livre non trouvé dans la bibliotheque\n");
                break;
            }
        };

        let livre = &mut bibliotheque.livres[index];
        
        afficher_tableau(&[livre.clone()], "Livre a changer le statut");
        let confirmation = input_valeur("Entrez (o/O) pour changer le statut> ");
        match confirmation.to_lowercase().trim() {
            "o" => {
                changer_statut_livre(livre);
                afficher_message_jaune("Statut du livre changé");
                afficher_tableau(&[livre.clone()], "Livre a changer le statut");
                break;
            },
            _ => {
                afficher_message_jaune("Operation annulée");
                afficher_tableau(&[livre.clone()], "Operation annulée");
                break;
            },
        
        }
    }
}