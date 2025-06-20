mod genr;
mod st_json;

use iced::widget::{button, row, column, container, text, svg, Space, text_input};
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
    NavigateTo(Pages), // Add navigation message
    // Form input messages
    PasswordNameChanged(String),
    PasswordChanged(String),
    WebsiteChanged(String),
    UsernameChanged(String),
    NotesChanged(String),
    SavePasswordDetails,
}

// Define the PasswordGenerator struct to hold the state of the password generator
#[derive(Default)]
struct PasswordGenerator {
    current_page: Pages, // Add current page state
    generated_password: String,
    // Form fields for password details
    password_name: String,
    saved_password: String,
    website: String,
    username: String,
    notes: String,
}

// The main entry point of the application
pub fn main() -> iced::Result {
    // Start the iced application with custom window settings
    iced::application("Saltr", update, view)
        .window_size(Size::new(380.0, 640.0)) // Phone-like aspect ratio
        .resizable(false) // Fixed size for clean design
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
            // Pre-fill the password field with generated password and navigate to AddDetails
            password_generator.saved_password = password_generator.generated_password.clone();
            password_generator.current_page = Pages::AddDetails;
            println!("Save button has been clicked");
        }
        Message::NavigateTo(page) => {
            password_generator.current_page = page;
            println!("Navigated to: {:?}", password_generator.current_page);
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
            println!("Saving password details:");
            println!("Name: {}", password_generator.password_name);
            println!("Password: {}", password_generator.saved_password);
            println!("Website: {}", password_generator.website);
            println!("Username: {}", password_generator.username);
            println!("Notes: {}", password_generator.notes);
            
            // Clear form fields after saving
            password_generator.password_name.clear();
            password_generator.saved_password.clear();
            password_generator.website.clear();
            password_generator.username.clear();
            password_generator.notes.clear();
            
            // Navigate back to main page
            password_generator.current_page = Pages::Current;
        }
    }
}






// Current page view 
fn view_current(password_generator: &PasswordGenerator) -> Element<Message> {
    let reload_svg = svg::Handle::from_path("assets/reload.svg"); 
    let copy_svg = svg::Handle::from_path("assets/copy.svg");
    
    // Top section with title
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

    // Reload button (smaller, icon only)
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

    // Copy button (smaller, icon only)
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

    // Password display with buttons on sides
    let password_section = container(
        column![
            text(":)")
                .size(14),
            
            // Row with reload button, password display, and copy button
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

            // Save button below the row with improved padding
            button(
                text("Save")
                    .size(16)
            )
            .on_press(Message::Save)
            .padding([15, 25]) // Improved padding: [vertical, horizontal]
            .width(100), // Slightly wider for better proportions
        ]
        .spacing(20) // Added spacing between elements
        .align_x(iced::Alignment::Center)
    )
    .padding(25)
    .width(Fill);

    // Navigation buttons
    let navigation = row![
        button("View Passwords").on_press(Message::NavigateTo(Pages::ViewPasswords)),
        Space::with_width(10),
        button("Settings").on_press(Message::NavigateTo(Pages::Settings)),
    ]
    .spacing(10);

    // Main layout
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

    // Outer container
    container(main_content)
        .padding(20)
        .width(Fill)
        .height(Fill)
        .into()
}






// Add Details page view
fn view_add_details(password_generator: &PasswordGenerator) -> Element<Message> {
    let content = column![
        text("Save Password Details")
            .size(24),
        Space::with_height(30),
        
        // Password Name field
        column![
            text("Password Name *")
                .size(14),
            text_input("e.g., Gmail Account", &password_generator.password_name)
                .on_input(Message::PasswordNameChanged)
                .padding(10)
                .width(300),
        ]
        .spacing(5),
        
        Space::with_height(15),
        
        // Password field (pre-filled with generated password)
        column![
            text("Password *")
                .size(14),
            text_input("Your password", &password_generator.saved_password)
                .on_input(Message::PasswordChanged)
                .padding(10)
                .width(300)
                .secure(true), // Hide password characters
        ]
        .spacing(5),
        
        Space::with_height(15),
        
        // Website/App field
        column![
            text("Website/App")
                .size(14),
            text_input("e.g., gmail.com", &password_generator.website)
                .on_input(Message::WebsiteChanged)
                .padding(10)
                .width(300),
        ]
        .spacing(5),
        
        Space::with_height(15),
        
        // Username/Email field
        column![
            text("Username/Email")
                .size(14),
            text_input("e.g., john@example.com", &password_generator.username)
                .on_input(Message::UsernameChanged)
                .padding(10)
                .width(300),
        ]
        .spacing(5),
        
        Space::with_height(15),
        
        // Notes field
        column![
            text("Notes")
                .size(14),
            text_input("Additional notes (optional)", &password_generator.notes)
                .on_input(Message::NotesChanged)
                .padding(10)
                .width(300),
        ]
        .spacing(5),
        
        Space::with_height(30),
        
        // Action buttons
        row![
            button("Cancel")
                .on_press(Message::NavigateTo(Pages::Current))
                .padding([10, 20]),
            Space::with_width(15),
            button("Save Password")
                .on_press(Message::SavePasswordDetails)
                .padding([10, 20]),
        ]
        .spacing(10),
        
        Space::with_height(20),
        text("* Required fields")
            .size(12),
    ]
    .spacing(0)
    .align_x(iced::Alignment::Center);

    container(content)
        .padding(40)
        .width(Fill)
        .height(Fill)
        .into()
}





// View Passwords page
fn view_passwords(_password_generator: &PasswordGenerator) -> Element<Message> {
    let content = column![
        text("Saved Passwords")
            .size(24),
        Space::with_height(20),
        text("No passwords saved yet")
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






// Settings page
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