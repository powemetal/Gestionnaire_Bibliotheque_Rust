use crate::livres::{Livre, LivreTemp};

pub fn conversion_livre_complet(livre: &LivreTemp) -> Result<Livre, String> {
    if !livre.est_complet() {
        return Err("Le livre est incomplet, veuillez remplir tous les champs.".into());
    }

    let livre = Livre {
        titre: livre.titre.clone().unwrap(),
        auteur: livre.auteur.clone().unwrap(),
        annee: livre.annee.unwrap(),
        pages: livre.pages.unwrap(),
        genre: livre.genre.clone().unwrap(),
        statut: livre.statut,
    };

    Ok(livre)
}
