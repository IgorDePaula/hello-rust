struct Pedido {
    id: u32,
    valor: f64,
}

fn buscar_pedido(id: u32) -> Option<Pedido> {
    let pedidos = vec![
        Pedido {
            id: 1,
            valor: 150.0,
        },
        Pedido {
            id: 2,
            valor: 300.0,
        },
    ];
    pedidos.into_iter().find(|p| p.id == id)
}

fn aplicar_desconto(valor: f64, desconto_pct: f64) -> Result<f64, String> {
    if desconto_pct >= 100.0 {
        return Err(format!("erro: desconto inválido"));
    }
    Ok(valor * (1.0 - desconto_pct / 100.0))
}

fn processar_pedido(id: u32, desconto_pct: f64) -> String {
    let pedido = match buscar_pedido(id) {
        Some(p) => p,
        None    => return "erro: pedido não encontrado".to_string(),
    };

    match aplicar_desconto(pedido.valor, desconto_pct) {
        Ok(total) => format!("total com desconto: {:.2}", total),
        Err(e)    => e,
    }
}

fn main() {
    println!("{}", processar_pedido(1, 10.0)); // sucesso: 135.0
    println!("{}", processar_pedido(2, 110.0)); // erro: desconto inválido
    println!("{}", processar_pedido(9, 10.0)); //
}
