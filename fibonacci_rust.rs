#![allow(warnings)]

use std::io;

fn main() {
    println!("máximo:");
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    println!();
    
    let mut anterior: u128 = 0;
    let mut atual: u128 = 1;
    let mut proximo: u128 = 0;
    let mut contador: u32 = 0;
    let mut contagem: u128 = input.trim().parse().expect("Por favor, insira um número válido");

    while true {
        contador += 1;
        proximo = anterior + atual;
        if proximo > contagem {
            break;
        }
        println!("{}º :{}",contador, proximo);
        anterior = atual;
        atual = proximo;

    }
    println!();
    println!("finalização");
}
