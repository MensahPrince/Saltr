// viewpasswords.rs - Fixed to use existing PasswordDetails from st_json
use std::{fs::File, io::Read};
use serde::{Serialize, Deserialize};
use crate::st_json::PasswordDetails; // Use the existing struct from st_json

#[derive(Serialize, Deserialize)]
struct PasswordFile {
    
    passwords: Vec<PasswordDetails>,
}

//This function loads passwords from a JSON file and returns a vector of PasswordDetails
// It handles the case where the file might not exist or is empty.
// If the file is empty, it returns an empty vector.
// If the file does not exist, it returns an empty vector as well.
// If the file exists and contains data, it deserializes the JSON into a vector of PasswordDetails.
pub fn load_passwords_from_json() -> Result<Vec<PasswordDetails>, Box<dyn std::error::Error>> {
    let mut file = File::open("passwords.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    if contents.trim().is_empty() {
        return Ok(Vec::new());
    }
    
    let data: PasswordFile = serde_json::from_str(&contents)?;
    Ok(data.passwords)
}



 // the function below was used for testing purposes only
 //Can be removed in production or replaced with a proper test suite when needed. 
 

  
pub fn rf_json() {
    match load_passwords_from_json() {
        Ok(passwords) => {
            println!("File contents loaded successfully:");
            for entry in passwords {
                println!("Deserialized Entry: \n Name: {} \n Value: {} \n Website: {} \n Username: {} \n Notes: {} \n Created_At: {}", 
                    entry.name, entry.value, entry.website, entry.username, entry.notes, entry.created_at);
            }
        }
        Err(e) => {
            println!("Error loading passwords: {}", e);
        }
    }
}
