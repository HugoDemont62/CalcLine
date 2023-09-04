use std::io;

fn main() {
    println!("Calculator in Rust");
    println!("Available operations :");
    println!("1. Addition");
    println!("2. Soustraction");
    println!("3. Multiplication");
    println!("4. Division");

    println!("5. To leave");

    loop {
        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read input");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter the first number : ");
                let mut num1 = String::new();
                io::stdin().read_line(&mut num1)
                    .expect("Failed to read input");
                let num1: f64 = num1.trim().parse()
                    .expect("Invalid entry");

                println!("Enter the second number : ");
                let mut num2 = String::new();
                io::stdin().read_line(&mut num2)
                    .expect("Failed to read input");
                let num2: f64 = num2.trim().parse()
                    .expect("Invalid entry");

                let result = num1 + num2;
                println!("Result : {}", result);
            }
            2 => {
                println!("Enter the first number : ");
                let mut num1 = String::new();
                io::stdin().read_line(&mut num1)
                    .expect("Failed to read input");
                let num1: f64 = num1.trim().parse()
                    .expect("Invalid Entry");

                println!("Enter the second number : ");
                let mut num2 = String::new();
                io::stdin().read_line(&mut num2)
                    .expect("Failed to read input");
                let num2: f64 = num2.trim().parse()
                    .expect("Invalid entry");

                let result: f64 = num1 - num2;
                println!("Result : {}", result);
            }
            3 => {
                println!("Enter the first number : ");
                let mut num1 = String::new();
                io::stdin().read_line(&mut num1)
                    .expect("Failed to read input");
                let num1: f64 = num1.trim().parse()
                    .expect("Invalid Entry");

                println!("Enter the second number : ");
                let mut num2 = String::new();
                io::stdin().read_line(&mut num2)
                    .expect("Failed to read input");
                let num2: f64 = num2.trim().parse()
                    .expect("Invalid entry");

                let result: f64 = num1 * num2;
                println!("Result : {}", result);
            }
            4 => {
                println!("Enter the first number : ");
                let mut num1 = String::new();
                io::stdin().read_line(&mut num1)
                    .expect("Failed to read input");
                let num1: f64 = num1.trim().parse()
                    .expect("Invalid Entry");

                println!("Enter the second number : ");
                let mut num2 = String::new();
                io::stdin().read_line(&mut num2)
                    .expect("Failed to read input");
                let num2: f64 = num2.trim().parse()
                    .expect("Invalid entry");

                let result: f64 = num1 / num2;
                println!("Result : {}", result);
            }
            5 => {
                println!("Au revoir !");
                return;
            }
            _ => println!("Option invalide. Réessayez."),
        }
    }
}
