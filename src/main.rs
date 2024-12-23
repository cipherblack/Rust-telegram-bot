use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::{Message, ParseMode};
use serde::{Serialize, Deserialize};
use sqlite::Connection;
use std::env;

#[derive(Serialize, Deserialize)]
struct User {
    id: i64,
    username: String,
    language: String,
}

#[derive(BotCommands)]
#[command(rename = "lowercase")]
enum Command {
    #[command(description = "Get user info")]
    Info,
    #[command(description = "Set user language")]
    SetLanguage(String),
    #[command(description = "Get stats of the bot")]
    Stats,
    #[command(description = "Send a photo")]
    SendPhoto,
}

async fn send_photo(bot: &Bot, chat_id: i64) -> Result<(), Box<dyn std::error::Error>> {
    bot.send_photo(chat_id, "https://example.com/path/to/photo.jpg")
        .parse_mode(ParseMode::Html)
        .caption("Here is a sample photo!")
        .await?;
    Ok(())
}

async fn save_user(conn: &Connection, user_id: i64, username: &str, language: &str) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "INSERT OR REPLACE INTO users (id, username, language) VALUES (?1, ?2, ?3);",
        &[&user_id.to_string(), &username, &language],
    )?;
    Ok(())
}

async fn get_user_info(conn: &Connection, user_id: i64) -> Result<User, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT id, username, language FROM users WHERE id = ?1")?;
    let user = stmt.query_row([user_id], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            language: row.get(2)?,
        })
    })?;
    Ok(user)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let bot = Bot::from_env().auto_send();

    // اتصال به پایگاه داده SQLite
    let conn = Connection::open("users.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT,
            language TEXT
        )",
    )?;

    // دریافت دستورات از کاربران
    teloxide::repl(bot, move |bot, msg| {
        let conn = conn.clone();

        async move {
            match msg.text() {
                Some("/start") => {
                    bot.send_message(msg.chat.id, "Welcome! Use /info for user info.")
                        .await?;
                }
                Some("/info") => {
                    let user = get_user_info(&conn, msg.from().id).await?;
                    bot.send_message(
                        msg.chat.id,
                        format!("User Info:\nID: {}\nUsername: {}\nLanguage: {}", user.id, user.username, user.language)
                    )
                    .await?;
                }
                Some("/setlanguage") => {
                    let language = msg.text().unwrap_or_else(|| "").to_string();
                    save_user(&conn, msg.from().id, &msg.from().username.unwrap_or_default(), &language).await?;
                    bot.send_message(msg.chat.id, format!("Language set to {}", language)).await?;
                }
                Some("/stats") => {
                    // آمار ربات را نمایش می‌دهیم (مثال: تعداد کاربران)
                    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users")?;
                    let count: i64 = stmt.query_row([], |row| row.get(0))?;
                    bot.send_message(msg.chat.id, format!("Bot has {} users.", count))
                        .await?;
                }
                Some("/sendphoto") => {
                    send_photo(&bot, msg.chat.id).await?;
                }
                _ => {
                    bot.send_message(msg.chat.id, "Invalid command. Use /info, /setlanguage, /stats, or /sendphoto.")
                        .await?;
                }
            }
            Ok(())
        }
    })
    .await?;

    Ok(())
}
