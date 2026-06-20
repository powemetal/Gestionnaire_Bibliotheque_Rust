use std::io::{stdin, stdout, Write};

pub fn demander_valeur_champ(prompt:&str) ->String {
    let mut valeur = String::new();
    print!("{prompt}");
    stdout().flush().unwrap();
    stdin().read_line(&mut valeur).expect("Erreur lors de la lecture de la ligne");
    valeur.trim_end().to_string()
}