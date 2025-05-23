use std::io;

pub fn get_input(prompt: &str) -> String {
    use std::io::Write;
    print!("{prompt} ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}