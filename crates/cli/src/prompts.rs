use std::process::exit;

use anyhow::Result;
use inquire::{
    validator::Validation, Confirm, CustomType, InquireError, Password, PasswordDisplayMode, Text,
};
use tracing::error;

pub const PROMPT_ADMIN_MESSAGE: &str = "Is she an admin user?";
pub const PROMPT_ACTIVE_USER_MESSAGE: &str = "Is she an active user?";
pub const PROMPT_BRANCH_DB_NAME_MESSAGE: &str =
    "Which database would you like to use for creating a branch?";
pub const PROMPT_BRANCH_DB_NAME_DELETE_MESSAGE: &str =
    "Which branch database would you like to delete?";
pub const PROMPT_BRANCH_NAME_MESSAGE: &str = "What is the branch name?";
pub const PROMPT_EXPIRATION_DATE_MESSAGE: &str = "What is the expiration date in milliseconds?";
pub const PROMPT_EMAIL_MESSAGE: &str = "What is her email?";
pub const PROMPT_NEW_EMAIL_MESSAGE: &str = "What is her new email?";
pub const PROMPT_TOKEN_NAME_MESSAGE: &str = "What is the token name?";
pub const PROMPT_WRITE_MESSAGE: &str = "Should have write permissions?";

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

pub fn integer_prompt(message: &str) -> Result<u64, InquireError> {
    CustomType::<u64>::new(message).prompt()
}

pub fn integer_optional_prompt(message: &str) -> Result<Option<u64>, InquireError> {
    let validator = |s: &str| {
        if s.is_empty() {
            return Ok(Validation::Valid);
        }

        match s.parse::<u64>() {
            Ok(_) => Ok(Validation::Valid),
            Err(_) => Ok(Validation::Invalid(
                "The value is not a valid integer".into(),
            )),
        }
    };

    let s = Text::new(&format!("{} (Optional)", message))
        .with_validator(validator)
        .prompt_skippable()?;

    match s {
        Some(s) => {
            if s.is_empty() {
                Ok(None)
            } else {
                Ok(Some(s.parse::<u64>().unwrap()))
            }
        }
        None => Ok(None),
    }
}

pub fn text_prompt(message: &str) -> Result<String, InquireError> {
    Text::new(message).prompt()
}

pub fn text_optional_prompt(message: &str) -> Result<Option<String>, InquireError> {
    Text::new(&format!("{} (Optional)", message)).prompt_skippable()
}
