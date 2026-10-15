use std::cmp::Ordering;
use std::io;
use rand::prelude::*;

fn main() {
    println!("Adivinhe o número!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Por favor digite um número:");

          let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler linha");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Seu palpite é: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Muito pequeno"),
            Ordering::Greater => println!("Muito grande"),
            Ordering::Equal => {
                 println!("Você acertou");
                 break;
            }
        }
    }
}
