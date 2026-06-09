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
    match buscar_post(3).await {
        Ok(post)  => println!("#{}: {}, {}", post.id, post.title, post.body),
        Err(e)    => println!("Erro: {}", e),
    }
}