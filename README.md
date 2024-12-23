
# Telegram Bot in Rust

This is a simple and fully functional Telegram bot built with the `teloxide` library in Rust. The bot interacts with users, stores information in a SQLite database, and includes essential features such as handling commands, sending media, and updating user data.

## Features
- Handle user commands (`/start`, `/info`, `/setlanguage`, `/stats`, `/sendphoto`).
- Store user data (ID, username, language) in an SQLite database.
- Send messages, including media (photos), to users.
- Display bot statistics (e.g., number of users).
- Interact with the bot using simple commands.
- Easily extensible for further features.

## Prerequisites
Ensure you have the following installed on your system:
- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- **SQLite**: Ensure you have SQLite installed for database interactions.
- **Python (optional)**: If you want to use dotenv files.

## Installation and Setup

### 1. Clone the Repository
Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/cipherblack/rust-telegram-bot.git
cd telegram-bot
```

### 2. Set up Dependencies
Ensure that your dependencies are installed:

```bash
cargo build
```

This command will fetch and build the necessary libraries.

### 3. Create a `.env` File
Create a `.env` file in the root of your project and add your bot token:

```
TELEGRAM_BOT_TOKEN=your-telegram-bot-token
```

### 4. Configure the Database (Optional)
The bot uses SQLite to store user data. If the database file does not exist, it will be automatically created when you run the bot.

### 5. Running the Bot
To run the bot, use the following command:

```bash
cargo run
```

This will start the bot and it will begin listening for commands.

## Commands

- `/start`: Displays a welcome message and instructions on how to use the bot.
- `/info`: Fetches and displays the user's information (ID, username, language).
- `/setlanguage <language>`: Updates the user's language preference in the database.
- `/stats`: Displays the bot statistics, such as the number of users.
- `/sendphoto`: Sends a sample photo to the user.

## Bot Structure

- **`main.rs`**: The main entry point of the bot. Handles all interactions with users, including command parsing and database management.
- **`Cargo.toml`**: The file that defines dependencies and project configuration.
- **`users.db`**: SQLite database file used for storing user data.

## Extending the Bot
You can extend this bot by adding new commands, integrating more APIs, or using a more complex database like PostgreSQL for larger applications.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments
- [teloxide](https://crates.io/crates/teloxide): The Rust library for building Telegram bots.
- [SQLite](https://www.sqlite.org/): Lightweight database used for user data storage.

