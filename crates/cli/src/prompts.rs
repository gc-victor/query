use anyhow::Result;
use cliclack::input;
use colored::Colorize;

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
