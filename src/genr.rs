use rand::{rng, Rng};
use rand::distr::Alphanumeric;

pub fn generate_password(password: &mut String, length: usize) {
    let rng = rng();
    let generated: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    
    *password = generated;
    println!("Generated password: {}", password);
}