use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Post {
    id: u32,
    title: String,
    body: String,
}

async fn buscar_post(id: u32) -> Result<Post, reqwest::Error> {
    let url = format!("https://jsonplaceholder.typicode.com/posts/{}", id);
    let post = reqwest::get(&url)
        .await?           // aguarda a requisição
        .json::<Post>()
        .await?;          // aguarda o parse do JSON
    Ok(post)
}

#[tokio::main]
async fn main() {
    // sequencial — faz uma de cada vez
    for id in 1..=3 {
        let post = buscar_post(id).await.unwrap();
        println!("{}: {}", post.id, post.title);
    }

    // paralelo — dispara todas ao mesmo tempo
    // equivale a goroutines + WaitGroup + channel no Go
    let handles: Vec<_> = (1..=3)
        .map(|id| tokio::spawn(buscar_post(id)))
        .collect();

    for handle in handles {
        match handle.await.unwrap() {
            Ok(post)  => println!("{}: {}, {}", post.id, post.title, post.body),
            Err(e)    => println!("Erro: {}", e),
        }
    }
}