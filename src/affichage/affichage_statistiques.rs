

pub fn affichage_statistiques((total_livres, total_pages, moyenne_pages, nombre_disponible, nombre_emprunte):(i32, i32, f64, i32, i32)) {


    println!();
    println!("{:^42}","+==========================================+");
    println!("{:^42}","|      Statistiques de la bibliothèque     |");
    println!("{:^42}","+==========================================+");
    println!();
    print!("{:>31}", format!(" Nombre total de livres:"));
    println!("{:>10}", total_livres);

    print!("{:>31}", format!(" Nombre total de pages:" ));
    println!("{:>10}", total_pages);

    print!("{:>31}", format!(" Moyenne de pages par livre:" ));
    println!("{:>10.2}", moyenne_pages);


    println!();
    println!("{:^42}","Nombre disponible    Nombre empruntés");
    println!("{:^21}{:^21}", nombre_disponible, nombre_emprunte);
    println!();
    println!("+==========================================+");
}
