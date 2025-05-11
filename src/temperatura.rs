use crate::get_input::get_input;

pub fn converter_temperatura() {
    loop {
        println!("A - Celsius para Fahrenheit");
        println!("B - Celsius para Kelvin");
        println!("C - Fahrenheit para Celsius");
        println!("D - Fahrenheit para Kelvin");
        println!("E - Kelvin para Celsius");
        println!("F - Kelvin para Fahrenheit");
        println!("S - Voltar");

        let escolha = get_input("Escolha uma opção:");

        match escolha.to_uppercase().as_str() {
            "A" => {
                let input = get_input("Insira a temperatura em Celsius:");
                match input.parse::<f64>() {
                    Ok(n) => {
                        let resultado = celsius_para_fahrenheit(n);
                        println!("Valor convertido: {}ºF", resultado);
                    }
                    Err(_) => println!("Valor inválido. Tente novamente."),
                }
            }
            "B" => {
                let input = get_input("Insira a temperatura em Celsius:");
                match input.parse::<f64>() {
                    Ok(n) => {
                        let resultado = celsius_para_kelvin(n);
                        println!("Valor convertido: {}k", resultado);
                    }
                    Err(_) => println!("Valor inválido. Tente novamente."),
                }
            }
            "C" => {
                let input = get_input("Insira a temperatura em Fahrenheit:");
                match input.parse::<f64>() {
                    Ok(n) => {
                        let resultado = fahrenheit_para_celsius(n);
                        println!("Valor convertido: {}ºC", resultado);
                    }
                    Err(_) => println!("Valor inválido. Tente novamente."),
                }
            }
            "D" => {
                let input = get_input("Insira a temperatura em Fahrenheit:");
                match input.parse::<f64>() {
                    Ok(n) => {
                        let resultado = fahrenheit_para_kelvin(n);
                        println!("Valor convertido: {}K", resultado);
                    }
                    Err(_) => println!("Valor inválido. Tente novamente."),
                }
            }
            "E" => {
                let input = get_input("Insira a temperatura em Kelvin:");
                match input.parse::<f64>() {
                    Ok(n) => {
                        let resultado = kelvin_para_celsius(n);
                        println!("Valor convertido: {}ºC", resultado);
                    }
                    Err(_) => println!("Valor inválido. Tente novamente."),
                }
            }
            "F" => {
                let input = get_input("Insira a temperatura em Kelvin:");
                match input.parse::<f64>() {
                    Ok(n) => {
                        let resultado = kelvin_para_fahrenheit(n);
                        println!("Valor convertido: {}ºF", resultado);
                    }
                    Err(_) => println!("Valor inválido. Tente novamente."),
                }
            }
            "S" => {
                println!("Saindo do conversor de temperatura");
                break;
            }
            _ => {
                println!("Escolha inválida");
            }
        }
    }
}

pub fn celsius_para_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

pub fn celsius_para_kelvin(c: f64) -> f64 {
    c + 273.15
}

pub fn fahrenheit_para_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn fahrenheit_para_kelvin(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0 + 273.15
}

pub fn kelvin_para_celsius(k: f64) -> f64 {
    k - 273.15
}

pub fn kelvin_para_fahrenheit(k: f64) -> f64 {
    (k - 273.15) * 9.0 / 5.0 + 32.0
}
