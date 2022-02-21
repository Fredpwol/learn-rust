use std::env;
fn main() {
    let variables = env::args();
    for arg in variables{
        println!("{}", arg);
    }
}
