pub fn affichage_option<T: ToString>(valeur: &Option<T>) -> String {
    match valeur {
        Some(v) => format!("\x1b[92m{}\x1b[0m", v.to_string()),
        None => "\x1b[93mVide\x1b[0m".to_string(),
    }
}