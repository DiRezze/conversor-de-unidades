mod comprimento;
mod get_input;
mod massa;
mod temperatura;

use crate::comprimento::converter_comprimento;
use crate::get_input::get_input;
use crate::temperatura::converter_temperatura;
use std::io::Write;
use std::{thread, time};

fn main() {
    loop {
        println!("--- Conversor de Unidades ---");
        println!("1. Temperatura (C ↔ F)");
        println!("2. Comprimento (m ↔ km)");
        println!("3. Massa (g ↔ kg)");
        println!("0. Sair");

        let escolha = get_input("Escolha uma opção");

        match escolha.as_str() {
            "1" => {
                println!("Opção escolhida: Conversor de Temperatura");
                converter_temperatura();
            }
            "2" => {
                println!("Opção escolhida: Conversor de Comprimento");
                converter_comprimento();
            }
            "3" => {
                println!("Opção escolhida: Conversor de Massa");
            }
            "0" => {
                print!("Saindo");
                for _ in 0..3 {
                    thread::sleep(time::Duration::from_millis(250));
                    print!(".");
                    std::io::stdout().flush().unwrap();
                }
                println!();
                break;
            }
            _ => println!("Escolha inválida"),
        }
    }
}
