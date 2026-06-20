use crate::livres::{Livre, LivreTemp};

pub fn conversion_livre_complet(temp: LivreTemp) -> Livre {
    Livre {
        titre: temp.titre.unwrap(),
        auteur: temp.auteur.unwrap(),
        annee: temp.annee.unwrap(),
        pages: temp.pages.unwrap(),
        genre: temp.genre.unwrap(),
        statut: temp.statut,
    }
}