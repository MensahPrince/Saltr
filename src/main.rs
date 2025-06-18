use iced::widget::{button, row, container, text, svg};
use iced::{Element, Fill, Size};

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
}

// The main entry point of the application
pub fn main() -> iced::Result {
    // Start the iced application with custom window settings
    iced::application("Saltr", update, view)
        .window_size(Size::new(400.0, 300.0)) // Set window size: width 400px, height 300px
        .resizable(true) // Allow window resizing
        .run()
}

// The update function handles messages and updates the state accordingly
fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Copy => {
            println!(
                "Copy action triggered. Current counter value: {}",
                counter.value
            );
            // Handle the copy action
        }
        Message::Reload => {
            println!("Reload action triggered.");
            // Handle the reload action
        }
    }
}

// The view function defines the UI layout and appearance
fn view(_counter: &Counter) -> Element<Message> {
    let reload_svg = svg::Handle::from_path("assets/reload.svg"); 
    let copy_svg = svg::Handle::from_path("assets/copy.svg");
    let reload_btn = button(svg::Svg::new(reload_svg))
        .on_press(Message::Reload)
        .padding(10);

    let copy_btn = button(svg::Svg::new(copy_svg))
        .on_press(Message::Copy)
        .padding(10);

    container(
        row![
            reload_btn,
            // Display the current value of the counter as text with font size 20
            text("Generate New Password").size(20),
            // Create a "Decrement" button that sends the Decrement message when pressed
            copy_btn,
        ]
        .spacing(10)
    )
    .padding(10)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}
