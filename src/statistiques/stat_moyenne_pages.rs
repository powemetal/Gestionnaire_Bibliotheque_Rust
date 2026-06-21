use crate::livres::bibliotheque::Bibliotheque;


pub fn stat_moyenne_pages(bibliotheque: &Bibliotheque) -> i32{
    let total_livres = bibliotheque.livres.len() as i32;
    if total_livres == 0 {
        return 0
    }
    let total_pages: i32 = bibliotheque.livres.iter().map(|l| l.pages).sum();
    total_pages / total_livres
    
}