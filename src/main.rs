use std::env;
fn main() {
    let mut args = env::args();
    args.next();
    println!("{}",brainfuck_rs::brainfuck(args.next(),args.next()).unwrap())
}
