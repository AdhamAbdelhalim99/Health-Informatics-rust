use rand::{distributions::Alphanumeric, Rng};
use std::env;

fn generate_password(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <password_length>", args[0]);
        return;
    }

    let length: usize = args[1].parse().unwrap_or(12);
    let password = generate_password(length);

    println!("Generated Password: {}", password);
}