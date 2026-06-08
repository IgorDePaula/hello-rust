fn buscar_usuario(id: u32) -> Option<String> {
    let banco = vec!["Alice", "Borodin", "Carol"];
    banco.get(id as usize).map(|s| s.to_string())
}

fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Divisão por zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    // Jeito 1: match explícito
    match buscar_usuario(1) {
        Some(nome) => println!("Encontrou: {}", nome),
        None       => println!("Não encontrou"),
    }

    // Jeito 2: if let — quando só te importa um dos casos
    if let Some(nome) = buscar_usuario(0) {
        println!("Usuário: {}", nome);
    }

    // Jeito 3: unwrap_or — valor padrão se for None
    let nome = buscar_usuario(99).unwrap_or("Anônimo".to_string());
    println!("{}", nome);

    // Result na prática
    match dividir(10.0, 3.0) {
        Ok(resultado) => println!("Resultado: {:.2}", resultado),
        Err(e)        => println!("Erro: {}", e),
    }
}