use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rquickjs::{function::Func, Ctx, Exception, Result as RQuickJsResult};

pub fn init(ctx: &Ctx) -> RQuickJsResult<()> {
    let globals = ctx.globals();

    globals.set("___send_email", Func::from(send_email))?;

    Ok(())
}

#[derive(serde::Deserialize, Default, Debug)]
struct SMTPConfig {
    server: String,
    username: String,
    password: String,
}

#[derive(serde::Deserialize, Default)]
struct Options {
    from: String,
    to: Vec<String>,
    subject: String,
    body: String,
    reply_to: Option<String>,
    cc: Option<Vec<String>>,
    bcc: Option<Vec<String>>,
    smtp_server: Option<String>,
    smtp_username: Option<String>,
    smtp_password: Option<String>,
}

fn send_email(ctx: Ctx<'_>, options: String) -> RQuickJsResult<String> {
    let options: Options = serde_json::from_str(&options)
        .map_err(|e| Exception::throw_syntax(&ctx, &format!("Invalid JSON: {}", e)))?;

    let email = match message(&options) {
        Ok(value) => value,
        Err(value) => return Err(Exception::throw_syntax(&ctx, &value.to_string())),
    };

    let SMTPConfig {
        server: smtp_server,
        username: smtp_username,
        password: smtp_password,
    } = match smtp_config(options) {
        Ok(value) => value,
        Err(value) => return Err(Exception::throw_syntax(&ctx, &value)),
    };

    let credentials = Credentials::new(smtp_username, smtp_password);
    let mailer = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .credentials(credentials)
        .build();

    let response = match mailer.send(&email) {
        Ok(v) => Ok(v),
        Err(e) => Err(Exception::throw_syntax(&ctx, &format!("Send {}", e))),
    }?;

    Ok(response.message().collect::<Vec<_>>().join("\n"))
}

fn smtp_config(options: Options) -> Result<SMTPConfig, String> {
    let smtp_server = if let Some(server) = options.smtp_server {
        server
    } else {
        std::env::var("QUERY_SMTP_SERVER").map_err(|_| "SMTP server is required".to_owned())?
    };
    let smtp_username = if let Some(username) = options.smtp_username {
        username
    } else {
        std::env::var("QUERY_SMTP_USERNAME").map_err(|_| "SMTP username is required".to_owned())?
    };
    let smtp_password = if let Some(password) = options.smtp_password {
        password
    } else {
        std::env::var("QUERY_SMTP_PASSWORD").map_err(|_| "SMTP password is required".to_owned())?
    };

    Ok(SMTPConfig {
        server: smtp_server,
        username: smtp_username,
        password: smtp_password,
    })
}

fn message(options: &Options) -> Result<Message, String> {
    if options.from.is_empty() {
        return Err("From field is required".to_owned());
    }
    if options.to.is_empty() {
        return Err("To field is required".to_owned());
    }
    if options.subject.is_empty() {
        return Err("Subject field is required".to_owned());
    }
    if options.body.is_empty() {
        return Err("Body field is required".to_owned());
    }

    let from: Mailbox = match options.from.parse() {
        Ok(value) => value,
        Err(e) => return Err(format!("From {}", e)),
    };

    let mut email_builder = Message::builder()
        .from(from)
        .subject(options.subject.clone());

    for address in options.to.iter() {
        let mbox: Mailbox = match address.parse() {
            Ok(value) => value,
            Err(e) => return Err(format!("To {}", e)),
        };
        email_builder = email_builder.to(mbox);
    }
    if let Some(address) = options.reply_to.iter().next() {
        let mbox: Mailbox = match address.parse() {
            Ok(value) => value,
            Err(e) => return Err(format!("Reply To {}", e)),
        };
        email_builder = email_builder.reply_to(mbox);
    }
    if let Some(addresses) = options.cc.iter().next() {
        for address in addresses {
            let mbox: Mailbox = match address.parse() {
                Ok(value) => value,
                Err(e) => return Err(format!("CC {}", e)),
            };

            email_builder = email_builder.cc(mbox);
        }
    }
    if let Some(addresses) = options.bcc.iter().next() {
        for address in addresses {
            let mbox: Mailbox = match address.parse() {
                Ok(value) => value,
                Err(e) => return Err(format!("BCC {}", e)),
            };
            email_builder = email_builder.bcc(mbox);
            email_builder = email_builder.keep_bcc();
        }
    }
    let email = match email_builder.body(options.body.clone()) {
        Ok(email) => email,
        Err(e) => return Err(format!("Body {}", e)),
    };
    Ok(email)
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn test_build_message() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec![
                "Jane Doe <jane.doe@example.com>".to_string(),
                "Ali Doe <ali.doe@example.com>".to_string(),
            ],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec![
                "cc@example.com".to_string(),
                "cc_1@example.com".to_string(),
            ]),
            bcc: Some(vec![
                "bcc@example.com".to_string(),
                "bcc_1@example.com".to_string(),
            ]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
        };

        let message = message(&options).unwrap();
        let headers = message.headers();

        assert_eq!(
            headers.get_raw("From").unwrap(),
            "John Doe <john.doe@example.com>"
        );

        let body = &message.formatted();
        let body = std::str::from_utf8(body).unwrap();
        let re = Regex::new(r"Hello, world!").unwrap();
        assert!(re.is_match(body));

        assert_eq!(headers.get_raw("Subject").unwrap(), "Hello");
        assert_eq!(
            headers.get_raw("To").unwrap(),
            "Jane Doe <jane.doe@example.com>, Ali Doe <ali.doe@example.com>"
        );
        assert_eq!(headers.get_raw("Reply-To").unwrap(), "reply@example.com");
        assert_eq!(
            headers.get_raw("Cc").unwrap(),
            "cc@example.com, cc_1@example.com"
        );
        assert_eq!(
            headers.get_raw("Bcc").unwrap(),
            "bcc@example.com, bcc_1@example.com"
        );
        assert_eq!(
            headers.get_raw("Content-Transfer-Encoding").unwrap(),
            "7bit"
        );
    }

    #[test]
    fn test_build_message_missing_from() {
        let options = Options {
            from: "".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
        };

        let result = message(&options);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "From field is required");
    }

    #[test]
    fn test_build_message_missing_to() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec![],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
        };

        let result = message(&options);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "To field is required");
    }

    #[test]
    fn test_build_message_missing_subject() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "".to_string(),
            body: "Hello, world!".to_string(),
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
        };

        let result = message(&options);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Subject field is required");
    }

    #[test]
    fn test_build_message_missing_body() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "Hello".to_string(),
            body: "".to_string(),
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
        };

        let result = message(&options);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Body field is required");
    }

    #[test]
    fn test_build_message_error_from_invalid_input() {
        let options = Options {
            from: "John Doe".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
        };

        let result = message(&options);

        assert!(result.is_err());
        assert_eq!(result.clone().err().unwrap(), "From Invalid input");
    }

    #[test]
    fn test_build_message_error_to_invalid_input() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec!["".to_string()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
        };

        let result = message(&options);

        assert!(result.is_err());
        assert_eq!(result.clone().err().unwrap(), "To Invalid input");
    }

    #[test]
    fn test_build_message_error_reply_to_invalid_input() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            reply_to: Some("".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
        };

        let result = message(&options);

        assert!(result.is_err());
        assert_eq!(result.clone().err().unwrap(), "Reply To Invalid input");
    }

    #[test]
    fn test_build_message_error_cc_invalid_input() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            reply_to: Some("jane.doe@example.com".to_string()),
            cc: Some(vec!["".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
        };

        let result = message(&options);

        assert!(result.is_err());
        assert_eq!(result.clone().err().unwrap(), "CC Invalid input");
    }

    #[test]
    fn test_build_message_error_bcc_invalid_input() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            reply_to: Some("jane.doe@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
        };

        let result = message(&options);

        assert!(result.is_err());
        assert_eq!(result.clone().err().unwrap(), "BCC Invalid input");
    }

    #[test]
    fn test_smtp_config() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            reply_to: Some("jane.doe@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["".to_string()]),
            smtp_server: Some("smtp.example.com".to_string()),
            smtp_username: Some("username".to_string()),
            smtp_password: Some("password".to_string()),
        };

        let SMTPConfig {
            server,
            username,
            password,
        } = smtp_config(options).unwrap();

        assert_eq!(server, "smtp.example.com");
        assert_eq!(username, "username");
        assert_eq!(password, "password");
    }

    #[test]
    fn test_smtp_config_missing_server() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            reply_to: Some("jane.doe@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["".to_string()]),
            smtp_server: None,
            smtp_username: Some("username".to_string()),
            smtp_password: Some("password".to_string()),
        };

        std::env::remove_var("QUERY_SMTP_SERVER");

        let result = smtp_config(options);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "SMTP server is required");
    }

    #[test]
    fn test_smtp_config_missing_username() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            reply_to: Some("jane.doe@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["".to_string()]),
            smtp_server: Some("smtp.example.com".to_string()),
            smtp_username: None,
            smtp_password: Some("password".to_string()),
        };

        std::env::remove_var("QUERY_SMTP_USERNAME");

        let result = smtp_config(options);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "SMTP username is required");
    }

    #[test]
    fn test_smtp_config_missing_password() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            reply_to: Some("jane.doe@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["".to_string()]),
            smtp_server: Some("smtp.example.com".to_string()),
            smtp_username: Some("username".to_string()),
            smtp_password: None,
        };

        std::env::remove_var("QUERY_SMTP_PASSWORD");

        let result = smtp_config(options);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "SMTP password is required");
    }
}
