fn tamanho(s: &String) -> usize {
    s.len()
}

fn adicionar_saudacao(s: &String) -> String {
    format!("Olá, {}!", s)
}

fn main() {
    let nome = String::from("Borodin");

    let tam = tamanho(&nome);
    let saudacao = adicionar_saudacao(&nome);

    println!("Nome tem {} caracteres", tam);
    println!("{}", saudacao);
}