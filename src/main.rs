fn primeira_palavra<'a>(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    let resultado;

        let frase = String::from("ownership é poderoso").to_string();
        resultado = primeira_palavra(&frase);

    println!("{}", resultado);
}