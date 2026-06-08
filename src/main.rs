#[derive(Debug)]
enum StatusPedido {
    Pendente,
    Processando { etapa: u32 },
    Entregue(String),  // String = código de rastreio
    Cancelado { motivo: String },
}

fn descricao(status: &StatusPedido) -> String {
    // complete o match aqui
   match status {
       StatusPedido::Pendente => format!("Pedido pendente"),
       StatusPedido::Processando { etapa } => format!("Pedido em processo, etapa {}", etapa),
       StatusPedido::Entregue(codigo_rastreio) => format!("Pedido entregue, código de rastreio: {}", codigo_rastreio),
       StatusPedido::Cancelado { motivo } => format!("Pedido cancelado, motivo: {}", motivo),
   }
}

fn main() {
    let pedidos = vec![
        StatusPedido::Pendente,
        StatusPedido::Processando { etapa: 2 },
        StatusPedido::Entregue("BR123456789".to_string()),
        StatusPedido::Cancelado { motivo: "Fora de estoque".to_string() },
    ];

    for p in &pedidos {
        println!("{}", descricao(p));
    }
}