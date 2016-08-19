//term_app

//colored term
//https://mmstick.gitbooks.io/rust-programming-phoronix-reader-how-to/content/chapter11.html

use std::io;
use std::env;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

enum Op {
    None,
    Plus,
    Minus,
    Divide,
    Multiply,
}

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
    }

    let reader = io::stdin();
    let mut result = String::new();
    println!("input:");
    //let bytes_read = reader.read_line(&mut result).unwrap();
    reader.read_line(&mut result).ok();
    println!("output:");
    println!("{}",result);

    result.clear();
    reader.read_line(&mut result).ok();

    let mut last_op = Op::None;
    let x = result.split_whitespace().fold(0, |mut res: isize, s: &str| {
        match s {
            "+" => last_op = Op::Plus,
            "-" => last_op = Op::Minus,
            "*" => last_op = Op::Multiply,
            "/" => last_op = Op::Divide,
            _ => {
                let x = s.parse::<isize>().ok().unwrap();
                res = match last_op {
                    Op::Plus => res + x,
                    Op::Minus => res - x,
                    Op::Divide => res / x,
                    Op::Multiply => res * x,
                    Op::None => x,
                }
            }
        };
        res
    });
    println!("{}",x);

    let path: &'static str = env!("PATH");
    println!("the $PATH variable at the time of compiling was: {}", path);
}
