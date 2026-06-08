use anyhow::Result;  // substitui Result<T, E> — o E vira anyhow::Error

fn processar(path: &str) -> Result<i32> {
    let conteudo = std::fs::read_to_string(path)?;  // qualquer erro é convertido automaticamente
    let numero = conteudo.trim().parse::<i32>()?;
    Ok(numero * 2)
}

fn main() {
    match processar("dados.txt") {
        Ok(n)  => println!("Resultado: {}", n),
        Err(e) => println!("Erro: {}", e),  // mostra a mensagem do erro original
    }
}