use std::io;

fn sum(a:i32, b:i32) -> i32 {
    a + b
}

fn mul(a:i32, b:i32) -> i32 {
    a * b
}

fn div(a:i32, b:i32) -> i32 {
    a / b
}

fn sub(a:i32, b:i32) -> i32 {
    a - b
}


fn get_number(position: String) -> i32 {
    loop {
        println!("Enter {position} number");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read");

        let number: i32 = match number.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Entera valid number!");
                continue
            }
        };
        return number;
    }
}

fn main() {

    let a: i32 = get_number(String::from("first"));
    let b: i32 = get_number(String::from("second"));

    println!("---------");
    println!("Calulator");
    println!("---------\n");
 
    loop {
        println!("Enter the operation: ");
        let mut operator = String::new();

        io::stdin()
            .read_line(&mut operator)
            .expect("failed to read");

        operator = operator.trim().to_string();

        let result = match operator.as_str() {
            "+" => sum(a, b),
            "*" => mul(a, b),
            "/" => div(a, b),
            "-" => sub(a, b),
            "q" => break,
            _ => {
                println!("Enter a valid operation!\n");
                continue
            },
        };
    
        println!("{a} {operator} {b} is equal {result}\n");
    }
    println!("Thanks, good bye!");
}
