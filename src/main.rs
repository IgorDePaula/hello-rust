use anyhow::{anyhow, Result};

struct Pedido {
    id: u32,
    valor: f64,
}

fn buscar_pedido(id: u32) -> Option<Pedido> {
    let pedidos = vec![
        Pedido { id: 1, valor: 150.0 },
        Pedido { id: 2, valor: 300.0 },
    ];
    pedidos.into_iter().find(|p| p.id == id)
}

fn aplicar_desconto(valor: f64, desconto_pct: f64) -> Result<f64> {
    if desconto_pct >= 100.0 {
        return Err(anyhow!("desconto inválido: {}%", desconto_pct));
    }
    Ok(valor * (1.0 - desconto_pct / 100.0))
}

fn processar_pedido(id: u32, desconto_pct: f64) -> Result<String> {
    let pedido = buscar_pedido(id).ok_or_else(|| anyhow!("pedido {} não encontrado", id))?;
    let total = aplicar_desconto(pedido.valor, desconto_pct)?;
    Ok(format!("total: {:.2}", total))
}

fn main() {
    let casos = vec![
        processar_pedido(1, 10.0),
        processar_pedido(2, 110.0),
        processar_pedido(9, 10.0),
    ];

    for resultado in casos {
        match resultado {
            Ok(s)  => println!("ok: {}", s),
            Err(e) => println!("erro: {}", e),
        }
    }
}