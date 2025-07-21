use std::{fs::File, io::Read};
use serde::{Serialize, Deserialize};

//A function to deserialize stored passwords from json
pub fn rf_json() {
    let mut file = File::open("passwords.json").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    println!("File contents: {}", contents);

    #[derive(Serialize, Deserialize, Debug)]
    struct PasswordDetails {
        name: String,
        value: String,
        website: String,
        username: String,
        notes: String,
        created_at: String,
    }
    #[derive(Serialize, Deserialize, Debug)]
    struct PasswordFile {
        passwords: Vec<PasswordDetails>,
    }

    // Deserialize into the wrapper struct
    let data: PasswordFile = serde_json::from_str(&contents)
        .expect("JSON was not well-formatted");

    // Print each password entry
    for entry in data.passwords {
        println!("Deserialized Entry: Name: {} \n Value: {} \n Website: {} \n Username: {} \n Notes: {} \n Created_At: {}", entry.name, entry.value, entry.website, entry.username, entry.notes, entry.created_at);

    }

    
}