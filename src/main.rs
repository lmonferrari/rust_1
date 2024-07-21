use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Vamos jogar um jogo!");
    let numero_secreto: i32 = rand::thread_rng().gen_range(1..10);
    println!("{}", numero_secreto);

    loop {
        let mut numero = String::new();

        println!("Digite um número entre 1 e 10:");
        io::stdin()
            .read_line(&mut numero)
            .expect("Falha ao ler entrada.");

        let numero: i32 = numero.trim().parse().unwrap_or_default();

        match numero.cmp(&numero_secreto) {
            Ordering::Equal => {
                println!("Acertou!");
                break;
            }
            Ordering::Greater => println!("Seu número é maior que o secreto."),
            Ordering::Less => println!("Seu número é menor e o secreto."),
        }
    }
    print!("Número secreto é: {}", numero_secreto);
}
