use std::io;

fn main() {
    println!("Calculatrice en Rust");
    println!("Opérations disponibles :");
    println!("1. Addition");
    println!("2. Soustraction");
    println!("3. Multiplication");
    println!("4. Division");
    println!("2. Quitter");

    loop {
        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Échec de la lecture de l'entrée");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Entrez le premier nombre : ");
                let mut num1 = String::new();
                io::stdin().read_line(&mut num1)
                    .expect("Échec de la lecture de l'entrée");
                let num1: f64 = num1.trim().parse()
                    .expect("Saisie invalide");

                println!("Entrez le deuxième nombre : ");
                let mut num2 = String::new();
                io::stdin().read_line(&mut num2)
                    .expect("Échec de la lecture de l'entrée");
                let num2: f64 = num2.trim().parse()
                    .expect("Saisie invalide");

                let result = num1 + num2;
                println!("Résultat : {}", result);
            }
            2 => {
                println!("Au revoir !");
                return;
            }
            _ => println!("Option invalide. Réessayez."),
        }
    }
}
