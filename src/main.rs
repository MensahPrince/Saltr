// main.rs - Fixed to work with your existing code structure

mod genr;
mod st_json;
mod viewpasswords;

use iced::widget::{button, row, column, container, text, svg, Space, text_input, scrollable};
use iced::{Element, Fill, Size};

// Define the pages enum
#[derive(Debug, Clone, Default)]
enum Pages {
    #[default]
    Current,
    AddDetails,
    ViewPasswords,
    Settings,
}

// Define the message enum to represent possible user actions
#[derive(Debug, Clone)]
enum Message {
    Copy,
    Reload,
    Save,
    NavigateTo(Pages),
    // Form input messages
    PasswordNameChanged(String),
    PasswordChanged(String),
    WebsiteChanged(String),
    UsernameChanged(String),
    NotesChanged(String),
    SavePasswordDetails,
    // New message for loading passwords
    LoadPasswordsFromFile,
}

// Define the PasswordGenerator struct to hold the state of the password generator
#[derive(Default)]
struct PasswordGenerator {
    current_page: Pages,
    generated_password: String,
    // Form fields for password details
    password_name: String,
    saved_password: String,
    website: String,
    username: String,
    notes: String,
    // Add status message for user feedback
    status_message: String,
    // Add field to store loaded passwords - using the existing PasswordDetails from st_json
    loaded_passwords: Vec<st_json::PasswordDetails>,
}

// The main entry point of the application
pub fn main() -> iced::Result {
    iced::application("Saltr", update, view)
        .window_size(Size::new(700.0, 600.0))
        .resizable(false)
        .run()
}

// The update function handles messages and updates the state accordingly
fn update(password_generator: &mut PasswordGenerator, message: Message) {
    match message {
        Message::Copy => {
            let mut clipboard = arboard::Clipboard::new().expect("Failed to create clipboard");
            clipboard.set_text(&password_generator.generated_password)
                .expect("Failed to set clipboard text");
            println!("Copied to clipboard");
        }
        Message::Reload => {
            genr::generate_password(&mut password_generator.generated_password, 16);
            println!("Reload button has been clicked");
        }
        Message::Save => {
            password_generator.saved_password = password_generator.generated_password.clone();
            password_generator.current_page = Pages::AddDetails;
            password_generator.status_message.clear();
            println!("Save button has been clicked");
        }
        Message::NavigateTo(page) => {
            // Load passwords when navigating to ViewPasswords page
            if matches!(page, Pages::ViewPasswords) {
                match viewpasswords::load_passwords_from_json() {
                    Ok(passwords) => {
                        password_generator.loaded_passwords = passwords;
                        password_generator.status_message = format!("Loaded {} passwords", password_generator.loaded_passwords.len());
                    }
                    Err(e) => {
                        password_generator.status_message = format!("Error loading passwords: {}", e);
                        password_generator.loaded_passwords.clear();
                    }
                }
            }
            password_generator.current_page = page;
            println!("Navigated to: {:?}", password_generator.current_page);
        }
        Message::LoadPasswordsFromFile => {
            match viewpasswords::load_passwords_from_json() {
                Ok(passwords) => {
                    password_generator.loaded_passwords = passwords;
                    password_generator.status_message = format!("Refreshed: {} passwords loaded", password_generator.loaded_passwords.len());
                }
                Err(e) => {
                    password_generator.status_message = format!("Error loading passwords: {}", e);
                    password_generator.loaded_passwords.clear();
                }
            }
        }
        // Handle form input changes
        Message::PasswordNameChanged(value) => {
            password_generator.password_name = value;
        }
        Message::PasswordChanged(value) => {
            password_generator.saved_password = value;
        }
        Message::WebsiteChanged(value) => {
            password_generator.website = value;
        }
        Message::UsernameChanged(value) => {
            password_generator.username = value;
        }
        Message::NotesChanged(value) => {
            password_generator.notes = value;
        }
        Message::SavePasswordDetails => {
            if password_generator.password_name.trim().is_empty() || 
               password_generator.saved_password.trim().is_empty() {
                password_generator.status_message = "Please fill in all required fields".to_string();
                return;
            }

            println!("Saving password details:");
            println!("Name: {}", password_generator.password_name);
            println!("Password: {}", password_generator.saved_password);
            println!("Website: {}", password_generator.website);
            println!("Username: {}", password_generator.username);
            println!("Notes: {}", password_generator.notes);
            
            let result = st_json::save_password_details_to_json(
                &password_generator.password_name,
                &password_generator.saved_password,
                &password_generator.website,
                &password_generator.username,
                &password_generator.notes,
                "passwords.json"
            );

            match result {
                Ok(_) => {
                    password_generator.status_message = "Password saved successfully!".to_string();
                    
                    // Clear form fields after successful save
                    password_generator.password_name.clear();
                    password_generator.saved_password.clear();
                    password_generator.website.clear();
                    password_generator.username.clear();
                    password_generator.notes.clear();
                }
                Err(e) => {
                    password_generator.status_message = format!("Error saving password: {}", e);
                }
            }
        }
    }

    // Still calling the test function for development
    viewpasswords::rf_json();
}

// Current page view (unchanged from your original)
fn view_current(password_generator: &PasswordGenerator) -> Element<Message> {
    let reload_svg = svg::Handle::from_path("assets/reload.svg"); 
    let copy_svg = svg::Handle::from_path("assets/copy.svg");
    
    let header = container(
        column![
            text("A password generator:")
                .size(20),
        ]
        .spacing(2)
        .align_x(iced::Alignment::Center)
    )
    .width(Fill)
    .padding(40);

    let reload_btn = button(
        container(
            svg::Svg::new(reload_svg).width(20).height(20)
        )
        .center_x(Fill)
        .center_y(Fill)
    )
    .on_press(Message::Reload)
    .padding(10)
    .width(40)
    .height(40);

    let copy_btn = button(
        container(
            svg::Svg::new(copy_svg).width(20).height(20)
        )
        .center_x(Fill)
        .center_y(Fill)
    )
    .on_press(Message::Copy)
    .padding(10)
    .width(40)
    .height(40);

    let password_section = container(
        column![
            text(":)")
                .size(14),
            
            row![
                reload_btn,
                Space::with_width(15),
                container(
                    text(if password_generator.generated_password.is_empty() { 
                        "Click to generate password" 
                    } else { 
                        &password_generator.generated_password 
                    })
                    .size(18)
                )
                .padding(20)
                .width(Fill),
                Space::with_width(15),
                copy_btn,
            ]
            .align_y(iced::Alignment::Center),

            button(
                text("Save")
                    .size(16)
            )
            .on_press(Message::Save)
            .padding([15, 25])
            .width(100),
        ]
        .spacing(20)
        .align_x(iced::Alignment::Center)
    )
    .padding(25)
    .width(Fill);

    let navigation = row![
        button("View Passwords").on_press(Message::NavigateTo(Pages::ViewPasswords)),
        Space::with_width(10),
        button("Settings").on_press(Message::NavigateTo(Pages::Settings)),
    ]
    .spacing(10);

    let main_content = column![
        header,
        Space::with_height(20),
        password_section,
        Space::with_height(20),
        navigation,
        Space::with_height(40),
    ]
    .spacing(0)
    .align_x(iced::Alignment::Center);

    container(main_content)
        .padding(20)
        .width(Fill)
        .height(Fill)
        .into()
}

// Add Details page view (unchanged from your original)
fn view_add_details(password_generator: &PasswordGenerator) -> Element<Message> {
    let mut content_items = vec![
        text("Save Password Details")
            .size(24)
            .into(),
        Space::with_height(30).into(),
    ];

    if !password_generator.status_message.is_empty() {
        let status_color = if password_generator.status_message.contains("successfully") {
            iced::Color::from_rgb(0.0, 0.6, 0.0)
        } else {
            iced::Color::from_rgb(0.8, 0.0, 0.0)
        };
        
        content_items.push(
            text(&password_generator.status_message)
                .size(14)
                .color(status_color)
                .into()
        );
        content_items.push(Space::with_height(15).into());
    }

    content_items.extend(vec![
        column![
            text("Password Name *")
                .size(14),
            text_input("e.g., Gmail Account", &password_generator.password_name)
                .on_input(Message::PasswordNameChanged)
                .padding(10)
                .width(300),
        ]
        .spacing(5)
        .into(),
        
        Space::with_height(15).into(),
        
        column![
            text("Password *")
                .size(14),
            text_input("Your password", &password_generator.saved_password)
                .on_input(Message::PasswordChanged)
                .padding(10)
                .width(300)
                .secure(true),
        ]
        .spacing(5)
        .into(),
        
        Space::with_height(15).into(),
        
        column![
            text("Website/App")
                .size(14),
            text_input("e.g., gmail.com", &password_generator.website)
                .on_input(Message::WebsiteChanged)
                .padding(10)
                .width(300),
        ]
        .spacing(5)
        .into(),
        
        Space::with_height(15).into(),
        
        column![
            text("Username/Email")
                .size(14),
            text_input("e.g., john@example.com", &password_generator.username)
                .on_input(Message::UsernameChanged)
                .padding(10)
                .width(300),
        ]
        .spacing(5)
        .into(),
        
        Space::with_height(15).into(),
        
        column![
            text("Notes")
                .size(14),
            text_input("Additional notes (optional)", &password_generator.notes)
                .on_input(Message::NotesChanged)
                .padding(10)
                .width(300),
        ]
        .spacing(5)
        .into(),
        
        Space::with_height(30).into(),
        
        row![
            button("Cancel")
                .on_press(Message::NavigateTo(Pages::Current))
                .padding([10, 20]),
            Space::with_width(15),
            button("Save Password")
                .on_press(Message::SavePasswordDetails)
                .padding([10, 20]),
        ]
        .spacing(10)
        .into(),
        
        Space::with_height(20).into(),
        text("* Required fields")
            .size(12)
            .into(),
    ]);

    let content = column(content_items)
        .spacing(0)
        .align_x(iced::Alignment::Center);

    container(content)
        .padding(40)
        .width(Fill)
        .height(Fill)
        .into()
}

// Enhanced View Passwords page that displays the loaded passwords
fn view_passwords(password_generator: &PasswordGenerator) -> Element<Message> {
    let mut content_items = vec![
        text("Saved Passwords")
            .size(24)
            .into(),
        Space::with_height(20).into(),
    ];

    // Show status message if any
    if !password_generator.status_message.is_empty() {
        let status_color = if password_generator.status_message.contains("loaded") || 
                             password_generator.status_message.contains("Refreshed") {
            iced::Color::from_rgb(0.0, 0.6, 0.0)
        } else {
            iced::Color::from_rgb(0.8, 0.0, 0.0)
        };
        
        content_items.push(
            text(&password_generator.status_message)
                .size(14)
                .color(status_color)
                .into()
        );
        content_items.push(Space::with_height(15).into());
    }

    // Display passwords if any are loaded
    if password_generator.loaded_passwords.is_empty() {
        content_items.extend(vec![
            text("No passwords found")
                .size(16)
                .into(),
            Space::with_height(10).into(),
            text("Save some passwords first!")
                .size(14)
                .into(),
        ]);
    } else {
        // Create a scrollable list of passwords
        let password_list: Vec<Element<Message>> = password_generator.loaded_passwords
            .iter()
            .map(|password| {
                container(
                    column![
                        // Password name (title)
                        text(&password.name)
                            .size(18)
                            .color(iced::Color::from_rgb(0.2, 0.6, 1.0)),
                        
                        Space::with_height(5),
                        
                        // Website (if provided)
                        if !password.website.is_empty() {
                            text(format!("Website: {}", password.website))
                                .size(14)
                        } else {
                            text("")
                                .size(14)
                        },
                        
                        // Username (if provided)
                        if !password.username.is_empty() {
                            text(format!("Username: {}", password.username))
                                .size(14)
                        } else {
                            text("")
                                .size(14)
                        },
                        
                        // Notes (if provided)
                        if !password.notes.is_empty() {
                            text(format!("Notes: {}", password.notes))
                                .size(14)
                        } else {
                            text("")
                                .size(14)
                        },
                        
                        // Password (hidden for security)
                        text(format!("Password: {}", "*".repeat(password.value.len())))
                            .size(14)
                            .color(iced::Color::from_rgb(0.6, 0.6, 0.6)),
                        
                        // Created date
                        text(format!("Created: {}", password.created_at))
                            .size(12)
                            .color(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                    ]
                    .spacing(3)
                )
                .padding(15)
                .width(Fill)
                .into()
            })
            .collect();

        content_items.push(
            scrollable(
                column(password_list)
                    .spacing(10)
            ).height(300)
             .width(Fill)
             .into()
        );
    }

    let reload_svg = svg::Handle::from_path("assets/reload.svg"); 

    // Navigation buttons
    content_items.extend(vec![
        Space::with_height(20).into(),
        row![
            button("<-")
                .on_press(Message::NavigateTo(Pages::Current))
                .padding([10, 20]),
            Space::with_width(15),
            button(
                container(
                    svg::Svg::new(reload_svg).width(20).height(20)
                )
            )
                .on_press(Message::LoadPasswordsFromFile)
                .padding([10, 20]),
        ]
        .spacing(10)
        .into(),
        
    ]);

    let content = column(content_items)
        .spacing(0)
        .align_x(iced::Alignment::Center);

    container(content)
        .padding(40)
        .width(Fill)
        .height(Fill)
        .into()
}

// Settings page (unchanged)
fn view_settings(_password_generator: &PasswordGenerator) -> Element<Message> { 
    let content = column![
        text("Settings")
            .size(24),
        Space::with_height(20),
        text("Configure your password generator")
            .size(16),
        Space::with_height(30),
        button("Back to Generator")
            .on_press(Message::NavigateTo(Pages::Current))
            .padding([10, 20]),
    ]
    .spacing(10)
    .align_x(iced::Alignment::Center);

    container(content)
        .padding(40)
        .width(Fill)
        .height(Fill)
        .into()
}

// Main view function - acts as a router
fn view(password_generator: &PasswordGenerator) -> Element<Message> {
    match password_generator.current_page {
        Pages::Current => view_current(password_generator),
        Pages::AddDetails => view_add_details(password_generator),
        Pages::ViewPasswords => view_passwords(password_generator),
        Pages::Settings => view_settings(password_generator),
    }
}