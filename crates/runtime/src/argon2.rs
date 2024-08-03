use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chacha20poly1305::aead::OsRng;
use rquickjs::{prelude::Func, Ctx, Error, Object, Result};

fn hash(password: String) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(e) => Err(Error::new_resolving_message(
            "argon2",
            "query:argon2",
            e.to_string(),
        )),
    }
}

fn verify(password: String, hash: String) -> Result<bool> {
    let parsed_hash = PasswordHash::new(&hash)
        .map_err(|e| Error::new_resolving_message("argon2", "query:argon2", e.to_string()))?;

    let is_password_valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(is_password_valid)
}

pub fn init(ctx: &Ctx) -> Result<()> {
    let globals = ctx.globals();

    let argon2 = Object::new(ctx.clone())?;

    argon2.set("hash", Func::from(hash))?;
    argon2.set("verify", Func::from(verify))?;

    globals.set("___argon2", argon2)?;

    Ok(())
}
