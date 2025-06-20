use serde::Serialize;
//use serde_json::Results;
use crate::genr::generate_password;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]

struct PasswordDetails{
    value: String,
    name: String,
    website: String,
    notes: String,
    email: String,
}


pub fn password_details() -> PasswordDetails {
    let mut password = String::new();
    generate_password(&mut password, 12);

    PasswordDetails {
        value: password,
        name: "Example Password Name".to_string(),
        website: "https://example.com".to_string(),
        notes: "This is an example password note".to_string(),
        email: "Sample Username".to_string(),
    }
}

pub fn save_password_to_json(file_path: &str) -> Result<(), std::io::Error> {
    let password_details = password_details();
    let json_data = serde_json::to_string_pretty(&password_details)
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut file = File::create(file_path)?;
    file.write_all(json_data.as_bytes())?;
    file.flush()?;
    Ok(())
}