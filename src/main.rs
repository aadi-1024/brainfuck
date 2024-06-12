use std::env::args;

mod tokens;
mod errors;
mod interpreter;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("provide filename");
        std::process::exit(1);
    }
    let mut i = interpreter::Interpreter::new(&args[1]).unwrap();
    i.run().unwrap();
}
