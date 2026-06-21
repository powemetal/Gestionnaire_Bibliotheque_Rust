use crate::livres::bibliotheque::Bibliotheque;

pub fn stat_nombre_livres(bibliotheque: &Bibliotheque) -> i32 {
    bibliotheque.livres.len() as i32
}