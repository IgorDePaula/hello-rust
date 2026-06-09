#[tokio::main]
async fn main() {
    println!("Iniciando...");
    let resultado = soma_async(2, 3).await;  // .await executa o Future
    println!("Resultado: {}", resultado);
}