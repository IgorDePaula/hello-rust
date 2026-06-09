fn main() {
    let produtos = vec![
        ("notebook", 3500.0_f64),
        ("mouse", 150.0),
        ("teclado", 400.0),
        ("monitor", 2100.0),
    ];

    // filter — filtra por condição
    let caros: Vec<&str> = produtos.iter()
        .filter(|(_, preco)| *preco > 500.0)
        .map(|(nome, _)| *nome)
        .collect();
    println!("{:?}", caros);  // ["notebook", "monitor"]

    // map — transforma cada elemento
    let precos_com_frete: Vec<f64> = produtos.iter()
        .map(|(_, preco)| preco * 1.1)
        .collect();
    println!("{:?}", precos_com_frete);

    // find — retorna Option com o primeiro que satisfaz
    let barato = produtos.iter().find(|(_, preco)| *preco < 200.0);
    match barato {
        Some((nome, preco)) => println!("Mais barato: {} — R${}", nome, preco),
        None => println!("Nenhum abaixo de R$200"),
    }

    // fold — equivalente ao reduce do PHP/JS
    let total: f64 = produtos.iter().fold(0.0, |acc, (_, preco)| acc + preco);
    println!("Total: R${}", total);

    // any / all
    let tem_barato = produtos.iter().any(|(_, preco)| *preco < 200.0);
    let todos_caros = produtos.iter().all(|(_, preco)| *preco > 100.0);
    println!("Tem barato: {} | Todos caros: {}", tem_barato, todos_caros);
}