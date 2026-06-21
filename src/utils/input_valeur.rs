use std::io::{stdin, stdout, Write};

pub fn input_valeur(prompt: impl AsRef<str>) ->String {
    let mut valeur = String::new();
    print!("{}", prompt.as_ref());
    stdout().flush().unwrap();
    stdin().read_line(&mut valeur).expect("Erreur lors de la lecture de la ligne");
    valeur.trim_end().to_string()
}