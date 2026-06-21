// Note-> Peut-etre faire un petit module pour affichage en couleurs
pub fn afficher_message_jaune(message: &str){
    println!("\x1b[33m{message}\x1b[0m"); 
}

pub fn afficher_message_vert(message: &str){
    println!("\x1b[32m{message}\x1b[0m"); 
}
