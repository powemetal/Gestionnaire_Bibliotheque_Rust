use crate::livres::bibliotheque::Bibliotheque;

use crate::statistiques::stat_disponible::stat_disponible;
use crate::statistiques::stat_emprunte::stat_emprunte;
use crate::statistiques::stat_total_pages::stat_total_pages;
use crate::statistiques::stat_moyenne_pages::stat_moyenne_pages;
use crate::statistiques::stat_total_livres::stat_nombre_livres;

use crate::utils::input_valeur::input_valeur;

use crate::affichage::affichage_statistiques::affichage_statistiques;

pub fn menu_stats(bibliotheque: &Bibliotheque) {
    loop {
        let total_livres = stat_nombre_livres(bibliotheque);
        let total_pages = stat_total_pages(bibliotheque);
        let moyenne_pages = stat_moyenne_pages(bibliotheque);
        let nombre_disponible = stat_disponible(bibliotheque);
        let nombre_emprunte = stat_emprunte(bibliotheque);

        let donnees = (total_livres, total_pages, moyenne_pages, nombre_disponible, nombre_emprunte,);

        affichage_statistiques(donnees);

        let _choix = input_valeur("Appuyez sur Entrée pour quitter...");
        break;
    }
}
