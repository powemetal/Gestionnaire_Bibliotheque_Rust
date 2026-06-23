use crate::livres::livre_temp::LivreTemp;
use crate::livres::enum_champ::Champ;


pub fn modifier_champ(livre: &mut LivreTemp, (champ, valeur): (Champ , String)){
    match champ {
        Champ::Titre => livre.titre = Some(valeur),
        Champ::Auteur => livre.auteur = Some(valeur),
        Champ::Annee => livre.annee = valeur.parse().ok(),
        Champ::Pages => livre.pages = valeur.parse().ok(),
        Champ::Genre => livre.genre = Some(valeur),
    }
}