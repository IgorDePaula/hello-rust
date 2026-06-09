fn main() {
    let nomes = vec!["Alice", "Bob", "Carol"];

    // iter() — empresta (&T), o vec continua existindo depois
    for nome in nomes.iter() {
        println!("{}", nome);  // nome é &&str aqui
    }
    println!("{:?}", nomes);  // ok — nomes ainda existe

    // into_iter() — move (T), o vec é consumido
    for nome in nomes.into_iter() {
        println!("{}", nome);  // nome é &str
    }
    // println!("{:?}", nomes);  // ERRO — nomes foi movido

    // iter_mut() — empresta mutável (&mut T)
    let mut valores = vec![1, 2, 3];
    for v in valores.iter_mut() {
        *v *= 2;  // modifica in-place
    }
    println!("{:?}", valores);  // [2, 4, 6]
}