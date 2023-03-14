use std::{
    env,
    ops::{Add, Div, Mul, Sub},
};

fn calc(a: i32, b: i32, operator: String) {
    let result = match operator.as_str() {
        "+" => a.add(b),
        "*" => a.mul(b),
        "/" => a.div(b),
        "-" => a.sub(b),
        "%" => a % b,
        "q" => return,
        _ => {
            eprintln!("Invalid operator!");
            return;
        }
    };

    println!("{a} {operator} {b} is equal {result}\n");
}
#[derive(Debug)]
struct Arguments {
    a: i32,
    b: i32,
    operator: String,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            println!("{args:?}");
            return Err("too much arguments");
        }

        let a: i32 = match args[1].trim().parse() {
            Ok(n) => n,
            Err(_) => panic!("Invalid number!"),
        };

        let operator = args[2].trim().to_string();

        let b: i32 = match args[3].trim().parse() {
            Ok(n) => n,
            Err(_) => panic!("Invalid number!"),
        };

        return Ok(Arguments {
            a: a,
            b: b,
            operator: operator,
        });
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match Arguments::new(&args) {
        Ok(a) => {
            calc(a.a, a.b, a.operator);
        }
        Err(e) => eprintln!("{e}"),
    };
}
