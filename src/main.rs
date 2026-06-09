struct Fila<T> {
    itens: Vec<T>
}

impl<T> Fila<T> {
    fn new() -> Self {
        Fila { itens: vec![] }
    }

    fn enfileirar(&mut self, item: T) {
       self.itens.push(item);
    }

    fn desenfileirar(&mut self) -> Option<T> {
        self.itens.pop()
    }

    fn tamanho(&self) -> usize {
        self.itens.len()
    }

    fn esta_vazia(&self) -> bool {
        self.tamanho() == 0
    }
}

fn main() {
    // teste com i32
    let mut fila_nums: Fila<i32> = Fila::new();
    fila_nums.enfileirar(10);
    fila_nums.enfileirar(20);
    fila_nums.enfileirar(30);
    println!("Tamanho: {}", fila_nums.tamanho());
    println!("Removeu: {:?}", fila_nums.desenfileirar());
    println!("Removeu: {:?}", fila_nums.desenfileirar());
    println!("Vazia: {}", fila_nums.esta_vazia());

    // teste com String
    let mut fila_nomes: Fila<String> = Fila::new();
    fila_nomes.enfileirar("Ana".to_string());
    fila_nomes.enfileirar("Borodin".to_string());
    println!("Primeiro: {:?}", fila_nomes.desenfileirar());
}