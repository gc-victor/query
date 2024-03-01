use std::env;

use anyhow::{anyhow, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chacha20poly1305::aead::OsRng;
use rustyscript::deno_core::{self, extension, op2};

#[op2]
#[string]
fn op_argon2_hash_extension(#[string] password: String) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    Ok(argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string())
}

#[op2]
#[string]
fn op_argon2_verify_extension(
    #[string] password: String,
    #[string] hash: String,
) -> Result<String> {
    let is_password_valid = Argon2::default()
        .verify_password(password.as_bytes(), &PasswordHash::new(&hash).unwrap())
        .is_ok();

    if is_password_valid {
        Ok("".to_string())
    } else {
        Err(anyhow!(
            "Error: The email or password is not correct.".to_string()
        ))
    }
}

extension!(
    init_argon2,
    ops = [op_argon2_hash_extension, op_argon2_verify_extension],
    esm_entry_point = "ext:init_argon2/init_argon2.js",
    esm = [ dir "src/controllers/functions/ext", "init_argon2.js" ],
);
