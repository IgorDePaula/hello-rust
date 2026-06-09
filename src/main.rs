trait Resumo {
    fn autor(&self) -> String;
    fn titulo(&self) -> String;

    // implementação padrão — pode ser sobrescrita
    fn chamariz(&self) -> String {
        format!("{}, por {} — leia mais...", self.titulo(), self.autor())
    }
}

struct Artigo {
    titulo: String,
    autor: String,
    conteudo: String,
}
impl Resumo for Artigo {
    fn autor(&self) -> String {
        self.autor.to_string()
    }

    fn titulo(&self) -> String {
        self.titulo.to_string()
    }

    fn chamariz(&self) -> String {
        format!("{}, por {} leia mais...{}", self.titulo(), self.autor(), self.conteudo)
    }
}

struct Tweet {
    usuario: String,
    mensagem: String,
}

impl Resumo for Tweet {
    fn autor(&self) -> String {
        self.usuario.to_string()
    }

    fn titulo(&self) -> String {
        self.mensagem.to_string()
    }
    fn chamariz(&self) -> String {
        format!("{}, por {} leia mais...", self.titulo(), self.autor())
    }
}

struct VideoAula {
    titulo: String,
    instrutor: String,
    duracao_min: u32,
}

impl Resumo for VideoAula {
    fn autor(&self) -> String {
        self.instrutor.to_string()
    }
    fn titulo(&self) -> String {
        self.titulo.to_string()
    }

    fn chamariz(&self) -> String {
        format!("{}, por {} leia mais...com duracao {}", self.titulo(), self.autor(), self.duracao_min)
    }
}

// implemente Resumo para os três tipos
// VideoAula deve sobrescrever chamariz() incluindo a duração

fn imprimir_resumo(item: &impl Resumo) {
    println!("{}", item.chamariz());
}

fn main() {
    let artigo = Artigo {
        titulo: "Ownership em Rust".to_string(),
        autor: "Borodin".to_string(),
        conteudo: "...".to_string(),
    };

    let tweet = Tweet {
        usuario: "borodin_dev".to_string(),
        mensagem: "Rust é incrível!".to_string(),
    };

    let aula = VideoAula {
        titulo: "Traits na prática".to_string(),
        instrutor: "Borodin".to_string(),
        duracao_min: 45,
    };

    imprimir_resumo(&artigo);
    imprimir_resumo(&tweet);
    imprimir_resumo(&aula);
}