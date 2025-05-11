use crate::get_input::get_input;

pub fn converter_comprimento() {
    loop{
        println!("A - Metros para polegadas");
        println!("B - Polegadas para metros");
        println!("S - Voltar");
        
        let escolha = get_input("Escolha uma opção:");

        match escolha.to_uppercase().as_str() {
            "A" => {
                let input = get_input("Insira o valor em metros:");
                match input.parse::<f32>() {
                    Ok(n) =>{
                        let resultado = metros_para_polegadas(n);
                        println!("{:.2} metros = {:.2} polegadas", n, resultado );
                        println!("-----------------------------------------");
                    }
                    Err(_) => println!("Valor inválido. Tente novamente."),
                }
            }
            "B" => {
                let input = get_input("Insira o valor em polegadas:");
                match input.parse::<f32>() {
                    Ok(n) =>{
                        let resultado = polegadas_para_metros(n);
                        println!("{:.2} polegadas = {:.2} metros", n, resultado );
                        println!("-----------------------------------------");
                    }
                    Err(_) => println!("Valor inválido. Tente novamente."),
                }
            }
            "S" => {
                println!("Saindo de conversor de comprimento");
                break;
            }
            _ => {
                println!("Escolha inválida");
            }
        }

    }
}

pub fn metros_para_polegadas (metros: f32) -> f32 {
    let convertido: f32 = metros * 39.37008;
    return convertido;
}

pub fn polegadas_para_metros (polegadas: f32) -> f32 {
    let convertido = polegadas * 0.0254;
    return convertido;
}
