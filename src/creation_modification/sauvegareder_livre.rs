use crate::bibliotheque::Bibliotheque;
use crate::livres::Livre;

pub fn sauvegarder_livre(livre:Livre, bibliotheque: &mut Bibliotheque) -> Result<(), String>{
    bibliotheque.livres.push(livre);
    Ok(())
    
}