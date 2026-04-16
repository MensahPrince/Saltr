// main.rs - Fixed to work with your existing code structure

mod genr;
mod st_json;
mod viewpasswords;

use iced::widget::{Space, button, column, container, row, scrollable, svg, text, text_input};
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
        .window_size(Size::new(900.0, 600.0))
        .resizable(false)
        .theme(|_| custom_theme())
        .run()
}

fn custom_theme() -> iced::Theme {
    iced::Theme::custom(
        "Obsidian Depth".to_string(),
        iced::theme::Palette {
            background: iced::Color::from_rgb8(0x0f, 0x0f, 0x0f),
            text: iced::Color::from_rgb8(0xf5, 0xf5, 0xf5),
            primary: iced::Color::from_rgb8(0x00, 0xd4, 0xaa),
            success: iced::Color::from_rgb8(0x00, 0xd4, 0xaa),
            danger: iced::Color::from_rgb8(0xff, 0x6b, 0x9d),
        }
    )
}

// The update function handles messages and updates the state accordingly
fn update(password_generator: &mut PasswordGenerator, message: Message) {
    match message {
        Message::Copy => {
            let mut clipboard = arboard::Clipboard::new().expect("Failed to create clipboard");
            clipboard
                .set_text(&password_generator.generated_password)
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
                        password_generator.status_message = format!(
                            "Loaded {} passwords",
                            password_generator.loaded_passwords.len()
                        );
                    }
                    Err(e) => {
                        password_generator.status_message =
                            format!("Error loading passwords: {}", e);
                        password_generator.loaded_passwords.clear();
                    }
                }
            }
            password_generator.current_page = page;
            println!("Navigated to: {:?}", password_generator.current_page);
        }
        Message::LoadPasswordsFromFile => match viewpasswords::load_passwords_from_json() {
            Ok(passwords) => {
                password_generator.loaded_passwords = passwords;
                password_generator.status_message = format!(
                    "Refreshed: {} passwords loaded",
                    password_generator.loaded_passwords.len()
                );
            }
            Err(e) => {
                password_generator.status_message = format!("Error loading passwords: {}", e);
                password_generator.loaded_passwords.clear();
            }
        },
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
            if password_generator.password_name.trim().is_empty()
                || password_generator.saved_password.trim().is_empty()
            {
                password_generator.status_message =
                    "Please fill in all required fields".to_string();
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
                "passwords.json",
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

// Current page view (Generator)
fn view_current(password_generator: &PasswordGenerator) -> Element<'_, Message> {
    let reload_svg = svg::Handle::from_path("assets/reload.svg");
    let copy_svg = svg::Handle::from_path("assets/copy.svg");
    let save_svg = svg::Handle::from_path("assets/save.svg");

    let reload_btn = button(
        container(svg::Svg::new(reload_svg).width(16).height(16))
            .center_x(Fill)
            .center_y(Fill),
    )
    .on_press(Message::Reload)
    .padding(8)
    .width(36)
    .height(36);

    let copy_btn = button(
        container(svg::Svg::new(copy_svg).width(16).height(16))
            .center_x(Fill)
            .center_y(Fill),
    )
    .on_press(Message::Copy)
    .padding(8)
    .width(36)
    .height(36);

    let password_display = row![
        container(
            text(if password_generator.generated_password.is_empty() {
                "Click to generate password"
            } else {
                &password_generator.generated_password
            })
            .size(16)
        )
        .padding(12)
        .style(|_theme: &iced::Theme| {
            container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb8(0x1a, 0x1a, 0x1a))),
                border: iced::Border {
                    color: iced::Color::from_rgb8(0x2a, 0x2a, 0x2a),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                text_color: Some(iced::Color::from_rgb8(0x00, 0xd4, 0xaa)),
                ..Default::default()
            }
        })
        .width(Fill),
        Space::with_width(8),
        reload_btn,
        Space::with_width(8),
        copy_btn,
    ]
    .align_y(iced::Alignment::Center);

    let save_btn = button(
        container(
            row![
                svg::Svg::new(save_svg).width(16).height(16),
                text("Save to Vault").size(14)
            ]
            .spacing(8)
        )
    )
    .on_press(Message::Save)
    .padding([10, 16]);

    let content = column![
        text("Generate Password").size(24),
        Space::with_height(24),
        password_display,
        Space::with_height(24),
        save_btn,
    ]
    .spacing(0)
    .max_width(500);

    container(content)
        .width(Fill)
        .height(Fill)
        .into()
}

// Add Details page view
fn view_add_details(password_generator: &PasswordGenerator) -> Element<'_, Message> {
    let mut content = column![
        text("Save Password Details").size(24),
        Space::with_height(30),
    ];

    if !password_generator.status_message.is_empty() {
        let status_color = if password_generator.status_message.contains("successfully") {
            iced::Color::from_rgb8(0x00, 0xd4, 0xaa)
        } else {
            iced::Color::from_rgb8(0xff, 0x6b, 0x9d)
        };

        content = content.push(
            text(&password_generator.status_message)
                .size(14)
                .color(status_color),
        );
        content = content.push(Space::with_height(24));
    }

    let form = column![
        column![
            text("Password Name *").size(14),
            text_input("e.g., Gmail Account", &password_generator.password_name)
                .on_input(Message::PasswordNameChanged)
                .padding(10),
        ]
        .spacing(8),
        column![
            text("Password *").size(14),
            text_input("Your password", &password_generator.saved_password)
                .on_input(Message::PasswordChanged)
                .padding(10)
                .secure(true),
        ]
        .spacing(8),
        column![
            text("Website/App").size(14),
            text_input("e.g., gmail.com", &password_generator.website)
                .on_input(Message::WebsiteChanged)
                .padding(10),
        ]
        .spacing(8),
        column![
            text("Username/Email").size(14),
            text_input("e.g., john@example.com", &password_generator.username)
                .on_input(Message::UsernameChanged)
                .padding(10),
        ]
        .spacing(8),
        column![
            text("Notes").size(14),
            text_input("Additional notes (optional)", &password_generator.notes)
                .on_input(Message::NotesChanged)
                .padding(10),
        ]
        .spacing(8),
    ]
    .spacing(20)
    .max_width(400);

    let actions = row![
        button("Cancel")
            .on_press(Message::NavigateTo(Pages::Current))
            .padding([10, 20]),
        button("Save Password")
            .on_press(Message::SavePasswordDetails)
            .padding([10, 20]),
    ]
    .spacing(12);

    content = content
        .push(form)
        .push(Space::with_height(32))
        .push(actions)
        .push(Space::with_height(16))
        .push(text("* Required fields").size(12).color(iced::Color::from_rgb8(0x88, 0x88, 0x88)));

    container(content.max_width(500))
        .width(Fill)
        .height(Fill)
        .into()
}

// Enhanced View Passwords page
fn view_passwords(password_generator: &PasswordGenerator) -> Element<'_, Message> {
    let reload_svg = svg::Handle::from_path("assets/reload.svg");

    let header_row = row![
        text("Saved Passwords").size(24),
        Space::with_width(Fill),
        button(container(svg::Svg::new(reload_svg).width(16).height(16)))
            .on_press(Message::LoadPasswordsFromFile)
            .padding(8),
    ]
    .align_y(iced::Alignment::Center);

    let mut content = column![
        header_row,
        Space::with_height(24),
    ];

    if !password_generator.status_message.is_empty() {
        let status_color = if password_generator.status_message.contains("loaded")
            || password_generator.status_message.contains("Refreshed")
        {
            iced::Color::from_rgb8(0x00, 0xd4, 0xaa)
        } else {
            iced::Color::from_rgb8(0xff, 0x6b, 0x9d)
        };

        content = content.push(
            text(&password_generator.status_message)
                .size(14)
                .color(status_color),
        );
        content = content.push(Space::with_height(16));
    }

    if password_generator.loaded_passwords.is_empty() {
        content = content.push(
            column![
                text("No passwords found").size(16),
                Space::with_height(8),
                text("Save some passwords first!").size(14).color(iced::Color::from_rgb8(0x88, 0x88, 0x88)),
            ]
        );
    } else {
        let password_list: Vec<Element<Message>> = password_generator
            .loaded_passwords
            .iter()
            .map(|password| {
                let mut col = iced::widget::Column::new()
                    .push(text(&password.name).size(16).font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }))
                    .push(Space::with_height(8));
                
                if !password.website.is_empty() {
                    col = col.push(text(format!("Website: {}", password.website)).size(14));
                }
                if !password.username.is_empty() {
                    col = col.push(text(format!("Username: {}", password.username)).size(14));
                }
                if !password.notes.is_empty() {
                    col = col.push(text(format!("Notes: {}", password.notes)).size(14));
                }
                
                col = col
                    .push(Space::with_height(12))
                    .push(
                        text(format!("Password: {}", "*".repeat(password.value.len())))
                            .size(14)
                            .color(iced::Color::from_rgb8(0x88, 0x88, 0x88))
                    );

                container(col)
                .padding(16)
                .width(Fill)
                .style(|_theme: &iced::Theme| {
                    container::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb8(0x1a, 0x1a, 0x1a))),
                        border: iced::Border {
                            color: iced::Color::from_rgb8(0x2a, 0x2a, 0x2a),
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                })
                .into()
            })
            .collect();

        content = content.push(
            scrollable(column(password_list).spacing(12))
                .height(Fill)
                .width(Fill),
        );
    }

    container(content.max_width(600))
        .width(Fill)
        .height(Fill)
        .into()
}

// Settings page
fn view_settings(_password_generator: &PasswordGenerator) -> Element<'_, Message> {
    let content = column![
        text("Settings").size(24),
        Space::with_height(24),
        text("Configure your password generator").size(14).color(iced::Color::from_rgb8(0x88, 0x88, 0x88)),
    ];

    container(content.max_width(500))
        .width(Fill)
        .height(Fill)
        .into()
}

// Main view function - acts as a router
fn view(password_generator: &PasswordGenerator) -> Element<'_, Message> {
    let sidebar = container(
        column![
            text("Saltr")
                .size(24)
                .font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() })
                .color(iced::Color::from_rgb8(0x00, 0xd4, 0xaa)),
            Space::with_height(32),
            column![
                button(text("Generator").width(Fill))
                    .on_press(Message::NavigateTo(Pages::Current))
                    .width(Fill)
                    .padding(10),
                button(text("Saved Passwords").width(Fill))
                    .on_press(Message::NavigateTo(Pages::ViewPasswords))
                    .width(Fill)
                    .padding(10),
                button(text("Settings").width(Fill))
                    .on_press(Message::NavigateTo(Pages::Settings))
                    .width(Fill)
                    .padding(10),
            ]
            .spacing(8)
        ]
    )
    .width(220)
    .height(Fill)
    .padding(24)
    .style(|_theme: &iced::Theme| {
        container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb8(0x1a, 0x1a, 0x1a))),
            border: iced::Border {
                color: iced::Color::from_rgb8(0x2a, 0x2a, 0x2a),
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    });

    let main_content = match password_generator.current_page {
        Pages::Current => view_current(password_generator),
        Pages::AddDetails => view_add_details(password_generator),
        Pages::ViewPasswords => view_passwords(password_generator),
        Pages::Settings => view_settings(password_generator),
    };

    let main_wrapper = container(main_content)
        .width(Fill)
        .height(Fill)
        .padding(40)
        .style(|_theme: &iced::Theme| {
            container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb8(0x0f, 0x0f, 0x0f))),
                ..Default::default()
            }
        });

    row![sidebar, main_wrapper].into()
}
