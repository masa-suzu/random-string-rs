use std::io::Write;

//extern crate random_string_rs;
use random_string_rs::generator::generate;
use random_string_rs::parser::{parse,Error};

fn main() {
    let mut seed = 0;

    loop {
        let input = match try_read_from_stdin::<String>() {
            Ok(x) => x,
            _ => std::process::exit(1),
        };
        match run(input,seed){
            Ok(s) => println!("{}",s),
            Err(e) => println!("{:?}",e),
        }
        seed = seed+1;
    }
}

fn try_read_from_stdin<T: std::str::FromStr>() -> Result<T, T::Err> {
    let mut s = String::new();
    std::io::stdout().write(b">> ").ok();
    std::io::stdout().lock().flush().ok();
    std::io::stdin().read_line(&mut s).ok();
    s.parse()
}

fn run(s: String,seed:u64) ->  Result<String,Error> {

    let p = match parse(&s){
        Ok(p) => p,
        Err(e) => return Err(e)
    };

    Ok(generate(p,seed))
}

