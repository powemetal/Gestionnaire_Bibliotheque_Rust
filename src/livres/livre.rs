use serde::Deserialize;
use crate::livres::statut::Statut;


#[derive(Deserialize, Clone, Default, PartialEq)]
#[allow(dead_code)]
#[serde(rename_all = "PascalCase")]
pub struct Livre {
    pub titre: String,
    pub auteur: String,
    #[serde(rename = "Année")]
    pub annee: i32,
    pub pages: i32,
    pub genre: String,
    pub statut: Statut,
}


pub trait AfficherStatut {
    fn afficher(&self) -> &str;
    fn afficher_inverse(&self) -> &str;
}

pub trait Affichable {
    fn afficher(&self);
}

impl AfficherStatut for Statut {
    fn afficher(&self) -> &str {
        match self {
            Statut::Disponible => "\x1b[92mDisponible\x1b[0m",
            Statut::Emprunte => "\x1b[91mEmprunté\x1b[0m",
        }
    }

    fn afficher_inverse(&self) -> &str {
        match self {
            Statut::Emprunte => "\x1b[92mDisponible\x1b[0m",
            Statut::Disponible => "\x1b[91mEmprunté\x1b[0m",
        }
    }
}

impl Affichable for Livre {
    fn afficher(&self) {
        println!(" Titre: {}", self.titre);
        println!("Auteur: {}", self.auteur);
        println!(" Année: {}", self.annee);
        println!(" Pages: {}", self.pages);
        println!(" Genre: {}", self.genre);
        println!("Statut: {}", self.statut.afficher());
    }
}

