use tokio::time::{sleep, Duration};

async fn tarefa(nome: &str, segundos: u64) {
    println!("{} iniciou", nome);
    sleep(Duration::from_secs(segundos)).await;
    println!("{} terminou", nome);
}

#[tokio::main]
async fn main() {
    // sequencial — total ~3s
    tarefa("A", 2).await;
    tarefa("B", 1).await;

    // concorrente — total ~2s (maior dos dois)
    // equivale ao goroutine + WaitGroup do Go
    let (_r1, _r2) = tokio::join!(
        tarefa("A join", 2),
        tarefa("B join", 1)
    );
}