use crate::livres::{LivreTemp, Statut};
use crate::creation_modification::champ::Champ;


pub fn modifier_champ(livre: &mut LivreTemp, (champ, valeur): (Champ , String)){
    match champ {
        Champ::Titre => livre.titre = Some(valeur),
        Champ::Auteur => livre.auteur = Some(valeur),
        Champ::Annee => livre.annee = valeur.parse().ok(),
        Champ::Pages => livre.pages = valeur.parse().ok(),
        Champ::Genre => livre.genre = Some(valeur),
        Champ::Statut => {
            livre.statut = match livre.statut {
                Statut::Disponible => Statut::Emprunte,
                Statut::Emprunte => Statut::Disponible,
            }
        }
    }
}