fn criar_saudador(nome: String) -> impl Fn() -> String {
    move || format!("Olá, {}!", nome)  // move captura `nome` por ownership
}

fn main() {
    let saudar = criar_saudador("Borodin".to_string());
    println!("{}", saudar());  // "Olá, Borodin!"
    println!("{}", saudar());  // pode chamar várias vezes
}