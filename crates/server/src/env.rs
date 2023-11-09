use std::env;

pub struct Env {}

impl Env {
    pub fn validate() {
        Self::token_secret();
        Self::dbs_path();
        Self::admin_email();
        Self::admin_password();
    }

    pub fn port() -> u16 {
        when_port()
    }

    pub fn dbs_path() -> String {
        when_dbs_path()
    }

    pub fn proxy() -> String {
        when_proxy()
    }

    pub fn proxy_port() -> String {
        when_proxy_port()
    }

    pub fn token_secret() -> String {
        when_token_secret()
    }

    pub fn admin_email() -> String {
        when_admin_email()
    }

    pub fn admin_password() -> String {
        when_admin_password()
    }
}

fn when_port() -> u16 {
    env::var("QUERY_SERVER_PORT")
        .unwrap_or("3000".to_string())
        .parse::<u16>()
        .unwrap()
}

fn when_dbs_path() -> String {
    env::var("QUERY_SERVER_DBS_PATH").unwrap_or("/mnt/dbs".to_string())
}

fn when_proxy() -> String {
    env::var("QUERY_SERVER_PROXY").unwrap_or("false".to_string())
}

fn when_proxy_port() -> String {
    env::var("QUERY_SERVER_PROXY_PORT").unwrap_or("3001".to_string())
}

fn when_token_secret() -> String {
    env::var("QUERY_SERVER_TOKEN_SECRET").expect("QUERY_SERVER_TOKEN_SECRET is not set")
}

fn when_admin_email() -> String {
    env::var("QUERY_SERVER_ADMIN_EMAIL").expect("QUERY_SERVER_ADMIN_EMAIL is not set")
}

fn when_admin_password() -> String {
    env::var("QUERY_SERVER_ADMIN_PASSWORD").expect("QUERY_SERVER_ADMIN_PASSWORD is not set")
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    fn before() {
        env::set_var("QUERY_SERVER_PORT", "3000");
        env::set_var("QUERY_SERVER_DBS_PATH", "path");
        env::set_var("QUERY_SERVER_TOKEN_SECRET", "secret");
        env::set_var("QUERY_SERVER_ADMIN_EMAIL", "email");
        env::set_var("QUERY_SERVER_ADMIN_PASSWORD", "password");
    }

    // TODO: Test Env::validate for each env case

    #[test]
    fn test_port() {
        before();

        env::set_var("QUERY_SERVER_PORT", "3000");

        assert_eq!(Env::port(), 3000);
    }

    #[test]
    fn test_port_with_default() {
        before();

        env::remove_var("QUERY_SERVER_PORT");

        assert_eq!(Env::port(), 3000);
    }

    #[test]
    fn test_dbs_path() {
        env::set_var("QUERY_SERVER_DBS_PATH", "path");

        assert_eq!(Env::dbs_path(), "path");
    }

    #[test]
    fn test_dbs_path_without_dbs_path() {
        before();

        env::remove_var("QUERY_SERVER_DBS_PATH");

        assert_eq!(Env::dbs_path(), "/mnt/dbs");
    }

    #[test]
    fn test_token_secret() {
        env::set_var("QUERY_SERVER_TOKEN_SECRET", "secret");

        assert_eq!(Env::token_secret(), "secret");
    }

    #[test]
    #[should_panic(expected = "QUERY_SERVER_TOKEN_SECRET is not set")]
    fn test_token_secret_without_token_secret() {
        before();

        env::remove_var("QUERY_SERVER_TOKEN_SECRET");

        Env::token_secret();
    }

    #[test]
    fn test_admin_email() {
        env::set_var("QUERY_SERVER_ADMIN_EMAIL", "email");

        assert_eq!(Env::admin_email(), "email");
    }

    #[test]
    #[should_panic(expected = "QUERY_SERVER_ADMIN_EMAIL is not set")]
    fn test_admin_email_without_admin_email() {
        before();

        env::remove_var("QUERY_SERVER_ADMIN_EMAIL");

        Env::admin_email();
    }

    #[test]
    fn test_admin_password() {
        env::set_var("QUERY_SERVER_ADMIN_PASSWORD", "password");

        assert_eq!(Env::admin_password(), "password");
    }

    #[test]
    #[should_panic(expected = "QUERY_SERVER_ADMIN_PASSWORD is not set")]
    fn test_admin_password_without_admin_password() {
        before();

        env::remove_var("QUERY_SERVER_ADMIN_PASSWORD");

        Env::admin_password();
    }
}
