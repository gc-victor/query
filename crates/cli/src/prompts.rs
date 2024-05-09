use std::process::exit;

use anyhow::Result;
use cliclack::input;
use colored::Colorize;
use inquire::{Confirm, InquireError, Password, PasswordDisplayMode, Text};
use tracing::error;

pub const PROMPT_ADMIN_MESSAGE: &str = "Is she an admin user?";
pub const PROMPT_ACTIVE_USER_MESSAGE: &str = "Is she an active user?";
pub const PROMPT_BRANCH_DB_NAME_MESSAGE: &str =
    "Which database would you like to use for creating a branch?";
pub const PROMPT_BRANCH_DB_NAME_DELETE_MESSAGE: &str =
    "Which branch database would you like to delete?";
pub const PROMPT_BRANCH_NAME_MESSAGE: &str = "What is the branch name?";
pub const PROMPT_EMAIL_MESSAGE: &str = "What is her email?";
pub const PROMPT_NEW_EMAIL_MESSAGE: &str = "What is her new email?";

pub fn password_prompt() -> Result<String, InquireError> {
    let password = Password::new("What is her password?")
        .without_confirmation()
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_formatter(&|s| {
            if s.is_empty() {
                return String::new();
            }
            String::from("Input received")
        })
        .prompt();

    if password.is_err() {
        error!("An error happened when asking for the password, try again.");
        exit(1);
    };

    password
}

pub fn new_password_prompt() -> Result<Option<String>, InquireError> {
    let password = Password::new("What is her new password? (Optional)")
        .without_confirmation()
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_formatter(&|s| {
            if s.is_empty() {
                return String::new();
            }
            String::from("Input received")
        })
        .prompt_skippable();

    if password.is_err() {
        error!("An error happened when asking for the password, try again.");
        exit(1);
    };

    password
}

pub fn confirm_prompt(message: &str) -> Result<bool, InquireError> {
    Confirm::new(message).with_default(true).prompt()
}

pub fn confirm_optional_prompt(message: &str) -> Result<Option<bool>, InquireError> {
    Confirm::new(&format!("{} (y/n) (Optional)", message)).prompt_skippable()
}

pub fn text_prompt(message: &str) -> Result<String, InquireError> {
    Text::new(message).prompt()
}

pub fn expiration_date() -> Result<Option<i64>> {
    let expiration_date: String = input(format!(
        "What is the expiration date in seconds? {}",
        "(Optional)".to_string().yellow()
    ))
    .placeholder("Enter a value in seconds.")
    .required(false)
    .validate(|input: &String| {
        if input.is_empty() {
            Ok(())
        } else {
            match input.parse::<i64>() {
                Ok(val) => {
                    if val < chrono::Utc::now().timestamp() {
                        Err("The date represented by the seconds should not be older than today.")
                    } else {
                        Ok(())
                    }
                }
                Err(_) => Err("Please enter a valid value in seconds."),
            }
        }
    })
    .interact()?;
    let expiration_date = if expiration_date.is_empty() {
        None
    } else {
        match expiration_date.parse::<i64>() {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    };

    Ok(expiration_date)
}
