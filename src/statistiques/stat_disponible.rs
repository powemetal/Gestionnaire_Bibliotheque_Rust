use crate::livres::bibliotheque::Bibliotheque;
use crate::livres::statut::Statut;

pub fn stat_disponible(bibliotheque: &Bibliotheque) -> i32{
    bibliotheque.livres.iter().filter(|l| l.statut == Statut::Disponible).count() as i32
    
}