use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // spawn lança a tarefa no background — não bloqueia
    let handle = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        42  // valor de retorno
    });

    println!("Continuou executando enquanto a tarefa roda");

    // espera terminar e pega o resultado — equivale ao WaitGroup + channel do Go
    let resultado = handle.await.unwrap();
    println!("Tarefa retornou: {}", resultado);
}