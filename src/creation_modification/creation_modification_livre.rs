use std::io::{stdin, stdout, Write};

use crate::affichage::affichage_mod_livre::affichage_mod_livre;
use crate::affichage::affichage::{afficher_message_jaune, afficher_message_vert};


use crate::bibliotheque::Bibliotheque;
use crate::creation_modification::modifier_champ::modifier_champ;
use crate::creation_modification::champ::Champ;
use crate::creation_modification::demander_valeur_champ::demander_valeur_champ;
use crate::creation_modification::conversion_livre_complet::conversion_livre_complet;
use crate::creation_modification::sauvegarder_livre::sauvegarder_livre; 


use crate::livres::{Livre, LivreTemp};

// use crate ::bibliotheque::Bibliotheque;

pub fn creation_livre(livre:Option<Livre>,  bibliotheque: &mut Bibliotheque){
    let mut livre_temp = match livre {
        
        Some(l) => LivreTemp {titre: Some(l.titre.clone()), auteur: Some(l.auteur.clone()), annee: Some(l.annee), pages: Some(l.pages), genre: Some(l.genre.clone()), statut: l.statut},
        None => LivreTemp::default(),
    };

    loop {
        affichage_mod_livre(&livre_temp);

        let mut choix = String::new();
        print!("Choisissez le champ a modifier > ");
        stdout().flush().unwrap();
        stdin().read_line(&mut choix).expect("Erreur lors de la lecture de la ligne.");
        
        match choix.to_lowercase().trim() {
            "1" => {
                let valeur = demander_valeur_champ("Entrez le nouveau Titre: ");
                modifier_champ(&mut livre_temp, (Champ::Titre, valeur));
            }
            "2" => {
                let valeur = demander_valeur_champ("Entrez le nouvel Auteur: ");
                modifier_champ(&mut livre_temp, (Champ::Auteur, valeur));
            }
            "3" => {
                let valeur = demander_valeur_champ("Entrez la nouvelle Année: ");
                match valeur.parse::<i32>() {
                    Ok(_) => modifier_champ(&mut livre_temp, (Champ::Annee, valeur)),
                    Err(_) => afficher_message_jaune("\nVeillez entrer un nombre"),
                };
                
            }
            "4" => {
                let valeur = demander_valeur_champ("Entrez le nouveau nombre de pages: ");
                match valeur.parse::<i32>() {
                    Ok(_) => modifier_champ(&mut livre_temp, (Champ::Pages, valeur)),
                    Err(_) => afficher_message_jaune("\nVeillez entrer un nombre"),
                }
                
            }
            "5" => {
                let valeur = demander_valeur_champ("Entrez le(s) nouveau genre(s): ");
                modifier_champ(&mut livre_temp, (Champ::Genre, valeur));
            }
            "s" => {
                let livre_complet = match conversion_livre_complet(&livre_temp) {
                    Ok(l) => l, 
                    Err(e) => {
                        afficher_message_jaune(&e);
                        continue;
                    }
                };
                sauvegarder_livre(livre_complet, bibliotheque);
                afficher_message_vert("Sauvegarde Reussie!");
                break;
            }
                
            

            "q" => {break;}
            _ => {afficher_message_jaune("\nCe choix est invalide");}
        }
    }
}



