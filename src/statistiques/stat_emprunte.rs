use crate::livres::bibliotheque::Bibliotheque;
use crate::livres::statut::Statut;

pub fn stat_emprunte(bibliotheque: &Bibliotheque) -> i32{
    bibliotheque.livres.iter().filter(|l| l.statut == Statut::Emprunte).count() as i32
    
}