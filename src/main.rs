use iced::widget::{button, column, text};
use iced::Element;

// Define the message enum to represent possible user actions
#[derive(Debug, Clone)]
enum Message {
    Increment, // Represents the action of incrementing the counter
    Decrement, // Represents the action of decrementing the counter 
}

// Define the Counter struct to hold the state of the counter
#[derive(Default)]
struct Counter {
    value: u64, // The current value of the counter
}

// The main entry point of the application
pub fn main() -> iced::Result {
    // Start the iced application with a title, update function, and view function
    iced::run("A cool Counter", update, view)
}

// The update function handles messages and updates the state accordingly
fn update(counter: &mut Counter, message: Message) {
    match message {
        // If the Increment message is received, increase the counter's value by 1
        Message::Increment => counter.value += 1,
        // Decrease the counter's value by 1 when the Decrement message is received
        Message::Decrement => counter.value -= 1,
    }
}

// The view function defines the UI layout and appearance
fn view(counter: &Counter) -> Element<Message> {
    column![
        // Display the current value of the counter as text with font size 20
        text(counter.value).size(20),
        // Create an "Increment" button that sends the Increment message when pressed
        button("Increment").on_press(Message::Increment),
        // Create a "Decrement" button that sends the Decrement message when pressed
        button("Decrement").on_press(Message::Decrement),
    ]
    .spacing(10) // Add spacing between the elements in the column
    .into()      // Convert the column into an Element<Message>
}