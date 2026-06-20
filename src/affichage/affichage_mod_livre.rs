use crate::livres::{LivreTemp, AfficherStatut};
use crate::affichage::affichage_option::{affichage_option};


pub fn affichage_mod_livre(livre: &LivreTemp){
    let titre = affichage_option(&livre.titre);
    let auteur = affichage_option(&livre.auteur);
    let annee = affichage_option(&livre.annee);
    let pages = affichage_option(&livre.pages);
    let genre = affichage_option(&livre.genre);
    // let statut = &livre.statut;




println!(
    "
Menu de creation/modification d'un livre
----------------------------------------
\x1b[97m1\x1b[0m-  Titre: {}
\x1b[97m2\x1b[0m- Auteur: {}
\x1b[97m3\x1b[0m-  Année: {}
\x1b[97m4\x1b[0m-  Pages: {}
\x1b[97m5\x1b[0m-  Genre: {}
   Statut: {}
----------------------------------------
(\x1b[97mS\x1b[0m)auvegarder, (\x1b[97mQ\x1b[0m)uitter",
    titre, auteur, annee, pages, genre, livre.statut.afficher());

}

