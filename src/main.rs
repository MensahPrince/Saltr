use iced::widget::{button, row, column, container, text, svg, Space};
use iced::{Element, Fill, Size, Border, Shadow, Vector};
use iced::{Color, Background, Theme};

// Define the message enum to represent possible user actions
#[derive(Debug, Clone)]
enum Message {
    Copy,
    Reload,
}

// Define the Counter struct to hold the state of the counter
#[derive(Default)]
struct Counter {
    value: u64, // The current value of the counter
    generated_password: String,
}

// The main entry point of the application
pub fn main() -> iced::Result {
    // Start the iced application with custom window settings
    iced::application("Saltr", update, view)
        .window_size(Size::new(380.0, 640.0)) // Phone-like aspect ratio
        .resizable(false) // Fixed size for clean design
        .theme(|_: &Counter| Theme::Dark) // Use dark theme
        .run()
}

// The update function handles messages and updates the state accordingly
fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Copy => {
            println!("Copy button has been clicked");
        }
        Message::Reload => {
            println!("Reload button has been clicked");
        }
    }
}

// Custom dark colors matching the reference
const DARK_BG: Color = Color::from_rgb(0.08, 0.08, 0.08);
const CARD_BG: Color = Color::from_rgb(0.12, 0.12, 0.12);
const ACCENT_COLOR: Color = Color::from_rgb(0.9, 0.7, 0.4);
const TEXT_PRIMARY: Color = Color::from_rgb(0.95, 0.95, 0.95);
const TEXT_SECONDARY: Color = Color::from_rgb(0.7, 0.7, 0.7);
const BUTTON_ACTIVE: Color = Color::from_rgb(0.2, 0.2, 0.2);

// The view function defines the UI layout and appearance
fn view(counter: &Counter) -> Element<Message> {
    let reload_svg = svg::Handle::from_path("assets/reload.svg"); 
    let copy_svg = svg::Handle::from_path("assets/copy.svg");
    
    // Top section with title
    let header = container(
        column![
            text("A password generator:")
                .size(20)
                .style(move |_theme: &Theme| {
                    text::Style {
                        color: Some(TEXT_PRIMARY),
                    }
                }),
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
    .height(40)
    .style(move |_theme: &Theme, status| {
        button::Style {
            background: Some(Background::Color(match status {
                button::Status::Hovered => Color::from_rgb(0.25, 0.25, 0.25),
                button::Status::Pressed => Color::from_rgb(0.15, 0.15, 0.15),
                _ => BUTTON_ACTIVE,
            })),
            text_color: TEXT_PRIMARY,
            border: Border {
                color: Color::from_rgb(0.3, 0.3, 0.3),
                width: 1.0,
                radius: 12.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
        }
    });

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
    .height(40)
    .style(move |_theme: &Theme, status| {
        button::Style {
            background: Some(Background::Color(match status {
                button::Status::Hovered => Color::from_rgb(0.95, 0.75, 0.45),
                button::Status::Pressed => Color::from_rgb(0.85, 0.65, 0.35),
                _ => ACCENT_COLOR,
            })),
            text_color: DARK_BG,
            border: Border {
                color: ACCENT_COLOR,
                width: 1.0,
                radius: 12.0.into(),
            },
            shadow: Shadow::default(),
        }
    });

    // Password display with buttons on sides
    let password_section = container(
        column![
            text(":)")
                .size(14)
                .style(move |_theme: &Theme| {
                    text::Style {
                        color: Some(TEXT_SECONDARY),
                    }
                }),
            
            // Row with reload button, password display, and copy button
            row![
                reload_btn,
                Space::with_width(15),
                container(
                    text(if counter.generated_password.is_empty() { 
                        "Password:" 
                    } else { 
                        &counter.generated_password 
                    })
                    .size(18)
                    .style(move |_theme: &Theme| {
                        text::Style {
                            color: Some(TEXT_PRIMARY),
                        }
                    })
                )
                .padding(20)
                .width(Fill)
                .style(move |_theme: &Theme| {
                    container::Style {
                        background: Some(Background::Color(DARK_BG)),
                        border: Border {
                            color: Color::from_rgb(0.25, 0.25, 0.25),
                            width: 1.0,
                            radius: 12.0.into(),
                        },
                        text_color: None,
                        shadow: Shadow::default(),
                    }
                }),
                Space::with_width(15),
                copy_btn,
            ]
            .align_y(iced::Alignment::Center),
        ]
        .spacing(0)
        .align_x(iced::Alignment::Center)
    )
    .padding(25)
    .width(Fill)
    .style(move |_theme: &Theme| {
        container::Style {
            background: Some(Background::Color(CARD_BG)),
            border: Border {
                color: Color::from_rgb(0.2, 0.2, 0.2),
                width: 1.0,
                radius: 20.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 20.0,
            },
            text_color: None,
        }
    });

    // Main layout
    let main_content = column![
        header,
        Space::with_height(20),
        password_section,
        Space::with_height(40),
    ]
    .spacing(0)
    .align_x(iced::Alignment::Center);

    // Outer container with dark background
    container(main_content)
        .padding(20)
        .width(Fill)
        .height(Fill)
        .style(move |_theme: &Theme| {
            container::Style {
                background: Some(Background::Color(DARK_BG)),
                ..Default::default()
            }
        })
        .into()
}