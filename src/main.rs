fn main() {
    let taxa = 0.1;  // variável no escopo externo

    // captura `taxa` automaticamente por referência
    let calcular_frete = |preco: f64| preco * taxa;

    println!("{}", calcular_frete(100.0));  // 10.0
    println!("{}", calcular_frete(250.0));  // 25.0
}