# Saltr 🔐

A simple and elegant password generator and password keeping app built with Rust and the Iced GUI framework.

## Features

- **Clean Modern UI**: Intuitive design (Coming Soon)
- **Password Generation**: Generate secure passwords with a single click ✅
- **Password Storage**: store your generated passwords. ✅
- **Password List View**: Provides a list of your stored passwords for Create, Read, Update and Delete Operations (Coming soon)
- **Cross-Platform**: Built with Rust for maximum compatibility ✅
- **Lightweight**: Fast and efficient with minimal resource usage ✅


## Installation

### Prerequisites

- Rust : rustc 1.85.0
- Cargo package manager

### Building from Source

Clone the repository:
  ```bash
  git clone https://github.com/MensahPrince/Saltr.git
  cd saltr
  cargo run
  ```

Dependencies:
   
   ```toml
   iced = { version = "0.13.1", features = ["svg"] }
   rand = "0.9.1"
   arboard = "3.5.0"
   serde = { version = "1.0.219", features = ["derive"] }
   serde_json = "1.0.140"
   dirs = "6.0.0"
   chrono = { version = "0.4", features = ["serde"] }
   ```

## Disclaimer:
During the production of this software, I was a novice to the Iced Framework. 
