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

    //println!("input:");
    ////let bytes_read = reader.read_line(&mut result).unwrap();
    //reader.read_line(&mut result).ok();

    println!("enter equation:");
    println!("{}",result);

    result.clear();
    reader.read_line(&mut result).ok();

    let mut last_op = Op::None;

    //let parsed = result.split_whitespace();
    //let x = parsed.iter().fold(0, |mut res: isize, s: &str| {
    //  match s {

    let parsed = result
        .chars()
        .fold(vec![String::new()], |mut res, chr|{
            match chr {
                '+' | '-' | '*' | '/' => {
                    println!("action: {}", chr);
                    res.push(chr.to_string());
                    res.push(String::new());
                },
                ' ' | '\r' | '\n' => {
                    //do_nothing
                },
                _ => {
                    println!("value: {}", chr);
                    res.last_mut().unwrap().push(chr);
                },
            }
            res
        });

    let x = parsed.iter().fold(0, |mut res: isize, s: &String| {
        match (*s).as_str() {
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

    let key = "PATH";
    match env::var(key) {
        Ok(val) => println!("the ${} variable at runtime was: : {:?}", key, val),
        Err(e) => println!("couldn't interpret {}: {}", key, e),
    }
}
