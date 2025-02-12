pub mod arguments;
mod config;
pub mod error;
mod opensubs;
mod utils;

use arguments::Arguments;
use colored::Colorize;
use config::Config;
use dialoguer::{Confirm, Input, Password};
use error::{Error, Result};
use opensubs::{get_os_token, use_opensubs};
use std::fs;

pub fn run(matches: Arguments) -> Result<String> {
    let config = Config::new()?;

    let mut current_lang: &str = &config.lang;
    let os_token: &str = &config.os_token;

    if let Some(lang) = matches.lang {
        current_lang = lang;
    }

    println!(
        "{} {}",
        "Current Language:".green(),
        current_lang.to_uppercase()
    );

    if matches.path.len() > 0 {
        if os_token.len() > 0 {
            return use_opensubs(matches.path, current_lang, os_token);
        } else {
            println!("{}", "[!] No OpenSubtitles token found.".red().bold());
            authenticate_os_user()?;
            return use_opensubs(matches.path, current_lang, os_token);
        }
    } else {
        println!("{}", "[!] No files provided.".red().bold());
    }

    return Ok("".to_string());
}

pub fn authenticate_os_user() -> Result<()> {
    let mut config = Config::new()?;

    if Confirm::new()
        .with_prompt("Do you have an OpenSubtitles token?")
        .interact()?
    {
        let token: String = Input::new()
            .with_prompt("Input your token")
            .interact_text()?;
        config.set_os_token(token)?
    } else {
        println!("You can generate a token with your username & password.");
        let username: String = Input::new().with_prompt("Username").interact_text()?;
        let password: String = Password::new().with_prompt("Password").interact()?;
        let token = get_os_token(&username, &password)?;
        config.set_os_token(token)?
    }

    Ok(())
}

pub fn save_file(content: &str, filename: &str) -> Result<()> {
    println!("{} {}", "Saving to".green(), filename.italic());
    fs::write(filename, content).map_err(|e| Error::IO(e))
}

#[cfg(test)]
mod tests {
    use super::opensubs::hash;

    #[test]
    fn test_hash_fn() {
        assert_eq!(&hash("./test/breakdance.avi").unwrap(), "8e245d9679d31e12");
    }
}
