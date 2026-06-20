use crate::bibliotheque::Bibliotheque;
use crate::livres::Livre;

pub fn sauvegarder_livre(livre:Livre, bibliotheque: &mut Bibliotheque) {
    bibliotheque.livres.push(livre);
}