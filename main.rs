use std::io;

fn main() {
    let numero_secreto = 42;
    let mut tentativas = 0;

    println!("=== JOGO DE ADIVINHAÇÃO ===");
    println!("Tente adivinhar um número de 1 a 100!");

    loop {
        println!("Digite seu palpite:");

        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Erro ao ler a entrada.");

        let palpite: i32 = match entrada.trim().parse() {
            Ok(numero) => numero,
            Err(_) => {
                println!("Digite apenas números.");
                continue;
            }
        };

        tentativas += 1;

        if palpite < numero_secreto {
            println!("O número secreto é maior.");
        } else if palpite > numero_secreto {
            println!("O número secreto é menor.");
        } else {
            println!(
                "Parabéns! Você acertou em {} tentativa(s)!",
                tentativas
            );
            break;
        }
    }
}
