use serde::{Serialize, Deserialize};
use std::fs::{File};
use std::io::{Write, Read};
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct PasswordDetails {
    pub name: String,
    pub value: String,
    pub website: String,
    pub username: String,
    pub notes: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordDatabase {
    pub passwords: Vec<PasswordDetails>,
}

impl Default for PasswordDatabase {
    fn default() -> Self {
        Self {
            passwords: Vec::new(),
        }
    }
}

// Function to create a PasswordDetails struct from form data
pub fn create_password_details(
    name: &str,
    password: &str,
    website: &str, 
    username: &str,
    notes: &str,
) -> PasswordDetails {
    use chrono::Utc;
    
    PasswordDetails {
        name: name.to_string(),
        value: password.to_string(),
        website: website.to_string(),
        username: username.to_string(),
        notes: notes.to_string(),
        created_at: Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
    }
}

// Function to load existing passwords from JSON file
pub fn load_password_database(file_path: &str) -> Result<PasswordDatabase, std::io::Error> {
    if !Path::new(file_path).exists() {
        return Ok(PasswordDatabase::default());
    }

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.trim().is_empty() {
        return Ok(PasswordDatabase::default());
    }

    let database: PasswordDatabase = serde_json::from_str(&contents)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    Ok(database)
}

// Function to save password database to JSON file
pub fn save_password_database(database: &PasswordDatabase, file_path: &str) -> Result<(), std::io::Error> {
    let json_data = serde_json::to_string_pretty(database)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut file = File::create(file_path)?;
    file.write_all(json_data.as_bytes())?;
    file.flush()?;
    Ok(())
}

// Main function to save password details to JSON (integrates with your UI)
pub fn save_password_details_to_json(
    name: &str,
    password: &str,
    website: &str,
    username: &str,
    notes: &str,
    file_path: &str,
) -> Result<(), std::io::Error> {
    // Load existing database
    let mut database = load_password_database(file_path)?;
    
    // Create new password entry
    let password_details = create_password_details(name, password, website, username, notes);
    
    // Add to database
    database.passwords.push(password_details);
    
    // Save updated database
    save_password_database(&database, file_path)?;
    
    println!("Password '{}' saved successfully to {}", name, file_path);
    Ok(())
}

// Function to get all saved passwords (useful for the "View Passwords" page)
pub fn get_all_passwords(file_path: &str) -> Result<Vec<PasswordDetails>, std::io::Error> {
    let database = load_password_database(file_path)?;
    Ok(database.passwords)
}

// Function to delete a password by name
pub fn delete_password(name: &str, file_path: &str) -> Result<bool, std::io::Error> {
    let mut database = load_password_database(file_path)?;
    let initial_len = database.passwords.len();
    
    database.passwords.retain(|p| p.name != name);
    
    if database.passwords.len() < initial_len {
        save_password_database(&database, file_path)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

// Example/test function (you can keep this for testing)
pub fn save_example_password_to_json(file_path: &str) -> Result<(), std::io::Error> {
    use crate::genr::generate_password;
    
    let mut password = String::new();
    generate_password(&mut password, 12);

    save_password_details_to_json(
        "Example Password Name",
        &password,
        "https://example.com",
        "Sample Username",
        "This is an example password note",
        file_path,
    )
}