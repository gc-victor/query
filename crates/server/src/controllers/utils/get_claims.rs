use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::env::Env;

use super::http_error::{unauthorized, HttpError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
}

#[instrument(err(Debug), skip(token))]
pub fn get_claims(token: &str) -> Result<Claims, HttpError> {
    let mut validation = Validation::new(Algorithm::default());

    validation.validate_exp = false;

    let decoded = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(Env::token_secret().as_ref()),
        &validation,
    ) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("{}", e.to_string());
            return Err(unauthorized());
        }
    };

    Ok(decoded.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    use dotenv::dotenv;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde_json::{from_str, json};

    #[test]
    fn test_get_claim_with_valid_token() {
        dotenv().ok();

        let claims = json!({"exp": 1_000_000_000, "iat": 1_000_000_000, "iss": "test"});
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(Env::token_secret().as_ref()),
        )
        .unwrap();

        let result = get_claims(&token);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().iss, "test");
    }

    #[test]
    fn test_get_claim_without_valid_token() {
        dotenv().ok();

        let result = get_claims("invalid_token");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), unauthorized());
    }

    #[test]
    fn test_the_debug() {
        let claims = Claims {
            exp: 1601234567,
            iat: 160123456,
            iss: "test-issuer".to_string(),
        };

        let expected_debug_string =
            "Claims { exp: 1601234567, iat: 160123456, iss: \"test-issuer\" }";

        assert_eq!(format!("{:?}", claims), expected_debug_string);
    }

    #[test]
    fn test_the_deserialize() {
        let claims_json = json!({
            "exp": 1601234567,
            "iat": 160123456,
            "iss": "test-issuer",
        });
        let claims: Claims = from_str(&claims_json.to_string()).unwrap();

        assert_eq!(claims.exp, 1601234567);
        assert_eq!(claims.iat, 160123456);
        assert_eq!(claims.iss, "test-issuer");
    }

    #[test]
    fn test_the_serialize() {
        let claims = Claims {
            exp: 1601234567,
            iat: 160123456,
            iss: "test-issuer".to_string(),
        };

        let expected_json = json!({
            "exp": 1601234567,
            "iat": 160123456,
            "iss": "test-issuer",
        });

        assert_eq!(serde_json::to_value(claims).unwrap(), expected_json);
    }
}
