mod creation_modification;
mod utils;
mod livres;
mod bibliotheque;
mod affichage;


// use menu::creation_livre;




use utils::validation;
use validation::valider_choix;

use creation_modification::creation_modification_livre::creation_livre;

use affichage::affichage::{afficher_menu_principal, afficher_tableau};


use crate::bibliotheque::Bibliotheque;


fn main() {
    let mut bibliotheque = Bibliotheque::charger();

    loop {
        afficher_menu_principal();
        let choix_utilisateur = valider_choix(6);
        match choix_utilisateur {
            Some(1) => {afficher_tableau(&bibliotheque.livres, "\nListe complete de la bibliotheque");},
            Some(2) => { creation_livre(None, &mut bibliotheque) },
            Some(3) => {},
            Some(4) => {},
            Some(5) => {},
            Some(6) => {break},
            _ => {},
        }
    }
    
}
