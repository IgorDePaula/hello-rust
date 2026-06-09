struct Pedido {
    id: u32,
    cliente: String,
    valor: f64,
    entregue: bool,
}

fn total_pendente(pedidos: &[Pedido]) -> f64 {
    pedidos
        .iter()
        .filter(|p| !p.entregue)   // só os não entregues
        .map(|p| p.valor)          // pega só o valor
        .sum()                     // soma tudo
}

fn clientes_vip(pedidos: &[Pedido], minimo: f64) -> Vec<String> {
    let mut clientes: Vec<String> = pedidos
        .iter()
        .filter(|p| p.valor > minimo)
        .map(|p| p.cliente.clone())  // precisa clonar — o Vec vai ser dono das Strings
        .collect();

    clientes.sort();                 // ordena alfabeticamente
    clientes.dedup();                // remove consecutivos duplicados (só funciona após sort)
    clientes

}

fn pedido_mais_valioso(pedidos: &[Pedido]) -> Option<u32> {
    pedidos
        .iter()
        .max_by(|a, b| a.valor.partial_cmp(&b.valor).unwrap())
        .map(|p| p.id)
}

fn main() {
    let pedidos = vec![
        Pedido { id: 1, cliente: "Ana".to_string(),     valor: 450.0,  entregue: false },
        Pedido { id: 2, cliente: "Borodin".to_string(), valor: 1200.0, entregue: false },
        Pedido { id: 3, cliente: "Ana".to_string(),     valor: 300.0,  entregue: true  },
        Pedido { id: 4, cliente: "Carlos".to_string(),  valor: 850.0,  entregue: false },
        Pedido { id: 5, cliente: "Borodin".to_string(), valor: 200.0,  entregue: true  },
    ];

    println!("Pendente: R${:.2}", total_pendente(&pedidos));
    println!("VIPs: {:?}", clientes_vip(&pedidos, 400.0));
    println!("Mais valioso: {:?}", pedido_mais_valioso(&pedidos));
}