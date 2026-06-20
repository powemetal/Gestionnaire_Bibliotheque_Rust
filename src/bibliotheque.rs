use crate::livres::{Livre}; 
// use serde::Deserialize;



pub struct Bibliotheque {
    pub livres: Vec<Livre>,
}


impl Bibliotheque {
    pub fn charger() -> Self {
        let data = std::fs::read_to_string("Data/livres.json").expect("Impossible de lire le fichier .json");    

        let livres: Vec<Livre> = serde_json::from_str(&data).expect("JSON invalide dans livres.json");

        Self {livres}
    }

    // pub fn ajouter_livre(&mut self, livre: Livre){
    //     self.livres.push(livre);
    // }

    // pub fn sauvegarder(){
    //     //plus tard
    // }
}


