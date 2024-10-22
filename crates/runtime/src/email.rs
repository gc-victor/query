use lettre::message::header::ContentType;
use lettre::message::{Attachment, Mailbox, MultiPart, SinglePart};
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use rquickjs::prelude::Async;
use rquickjs::{function::Func, Ctx, Exception, Result as RQuickJsResult};
use tracing::info;

pub fn init(ctx: &Ctx) -> RQuickJsResult<()> {
    let globals = ctx.globals();

    globals.set(
        "___send_email",
        Func::from(Async(|ctx, options| async move {
            send_email(ctx, options).await
        })),
    )?;

    Ok(())
}

#[derive(serde::Deserialize, Debug)]
struct SMTPConfig {
    server: String,
    username: String,
    password: String,
    protocol: Protocol,
}

#[derive(serde::Deserialize, Default, Debug)]
struct FileAttachment {
    filename: String,
    content: Vec<u8>,
    content_type: String,
}

#[derive(serde::Deserialize, Default, Debug)]
struct FileInline {
    content: Vec<u8>,
    content_type: String,
    content_id: String,
}

#[derive(serde::Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
enum Protocol {
    Smtp,
    Smtps,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Smtp => write!(f, "smtp"),
            Protocol::Smtps => write!(f, "smtps"),
        }
    }
}

#[derive(serde::Deserialize, Default, Debug)]
struct Options {
    from: String,
    to: Vec<String>,
    subject: String,
    body: String,
    reply_to: Option<String>,
    cc: Option<Vec<String>>,
    bcc: Option<Vec<String>>,
    attachment: Option<Vec<FileAttachment>>,
    // The image `attachment` will display inline into the email. E.g. <img src="cid:123">
    file_inline: Option<Vec<FileInline>>,
    smtp_server: Option<String>,
    smtp_username: Option<String>,
    smtp_password: Option<String>,
    smtp_protocol: Option<Protocol>,
}

async fn send_email(ctx: Ctx<'_>, options: String) -> RQuickJsResult<String> {
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
        protocol: smtp_protocol,
    } = match smtp_config(options) {
        Ok(value) => value,
        Err(value) => return Err(Exception::throw_syntax(&ctx, &value)),
    };

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::from_url(&format!(
            "{smtp_protocol}://{smtp_username}:{smtp_password}@{smtp_server}"
        ))
        .unwrap()
        .build();

    let response = match mailer.send(email).await {
        Ok(v) => Ok(v),
        Err(e) => Err(Exception::throw_syntax(
            &ctx,
            &format!("Email not sent: {}", e),
        )),
    }?;

    info!("Email sent: {:?}", response);

    Ok(format!(
        r#"{{"code": "{}", "message": "{}"}}"#,
        response.code(),
        response.message().collect::<Vec<_>>().join("\n")
    ))
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
        protocol: options.smtp_protocol.unwrap_or(Protocol::Smtps),
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

    let mut message_builder = Message::builder()
        .from(from)
        .subject(options.subject.clone())
        .header(ContentType::TEXT_HTML);

    for address in options.to.iter() {
        let mbox: Mailbox = match address.parse() {
            Ok(value) => value,
            Err(e) => return Err(format!("To {}", e)),
        };
        message_builder = message_builder.to(mbox);
    }
    if let Some(address) = options.reply_to.iter().next() {
        let mbox: Mailbox = match address.parse() {
            Ok(value) => value,
            Err(e) => return Err(format!("Reply To {}", e)),
        };
        message_builder = message_builder.reply_to(mbox);
    }
    if let Some(addresses) = options.cc.iter().next() {
        for address in addresses {
            let mbox: Mailbox = match address.parse() {
                Ok(value) => value,
                Err(e) => return Err(format!("CC {}", e)),
            };

            message_builder = message_builder.cc(mbox);
        }
    }
    if let Some(addresses) = options.bcc.iter().next() {
        for address in addresses {
            let mbox: Mailbox = match address.parse() {
                Ok(value) => value,
                Err(e) => return Err(format!("BCC {}", e)),
            };
            message_builder = message_builder.bcc(mbox);
            message_builder = message_builder.keep_bcc();
        }
    }
    if options.attachment.is_none() && options.file_inline.is_none() {
        return message_builder
            .body(options.body.clone())
            .map_err(|e| format!("Body {}", e));
    }

    let body_part = SinglePart::html(options.body.clone());
    let mut related_part = MultiPart::related().singlepart(body_part);

    if let Some(inline_attachments) = &options.file_inline {
        for inline in inline_attachments {
            let inline_attachment = Attachment::new_inline(inline.content_id.clone()).body(
                inline.content.clone(),
                inline
                    .content_type
                    .parse()
                    .map_err(|e| format!("Content-Type {}", e))?,
            );
            related_part = related_part.singlepart(inline_attachment);
        }
    }

    let alternative_part = MultiPart::alternative().multipart(related_part);
    let mut mixed_part = MultiPart::mixed().multipart(alternative_part);

    if let Some(attachments) = &options.attachment {
        for attachment in attachments {
            let regular_attachment = Attachment::new(attachment.filename.clone()).body(
                attachment.content.clone(),
                attachment
                    .content_type
                    .parse()
                    .map_err(|e| format!("Content-Type {}", e))?,
            );
            mixed_part = mixed_part.singlepart(regular_attachment);
        }
    }

    message_builder
        .multipart(mixed_part)
        .map_err(|e| format!("Multipart {}", e))
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
            attachment: None,
            file_inline: None,
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
            smtp_protocol: None,
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
            attachment: None,
            file_inline: None,
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
            smtp_protocol: None,
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
            attachment: None,
            file_inline: None,
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
            smtp_protocol: None,
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
            attachment: None,
            file_inline: None,
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
            smtp_protocol: None,
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
            attachment: None,
            file_inline: None,
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
            smtp_protocol: None,
        };

        let result = message(&options);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Body field is required");
    }

    #[test]
    fn test_build_message_with_attachments_and_inline_files() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            attachment: Some(vec![FileAttachment {
                filename: "test.txt".to_string(),
                content: b"Test file".to_vec(),
                content_type: "text/plain".to_string(),
            }]),
            file_inline: Some(vec![FileInline {
                content: b"Inline image".to_vec(),
                content_type: "image/png".to_string(),
                content_id: "123".to_string(),
            }]),
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
            smtp_protocol: None,
        };

        let message = message(&options).unwrap();
        let body = &message.formatted();
        let body = std::str::from_utf8(body).unwrap();

        assert!(body.contains("Hello, world!"));
        assert!(body.contains("Content-Type: text/plain"));
        assert!(body.contains("Content-Type: image/png"));
    }

    #[test]
    fn test_build_message_with_attachments_only() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            attachment: Some(vec![FileAttachment {
                filename: "test.txt".to_string(),
                content: b"Test file".to_vec(),
                content_type: "text/plain".to_string(),
            }]),
            file_inline: None,
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
            smtp_protocol: None,
        };

        let message = message(&options).unwrap();
        let body = &message.formatted();
        let body = std::str::from_utf8(body).unwrap();

        assert!(body.contains("Hello, world!"));
        assert!(body.contains("Content-Type: text/plain"));
    }

    #[test]
    fn test_build_message_with_inline_files_only() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            attachment: None,
            file_inline: Some(vec![FileInline {
                content: b"Inline image".to_vec(),
                content_type: "image/png".to_string(),
                content_id: "123".to_string(),
            }]),
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
            smtp_protocol: None,
        };

        let message = message(&options).unwrap();
        let body = &message.formatted();
        let body = std::str::from_utf8(body).unwrap();

        assert!(body.contains("Hello, world!"));
        assert!(body.contains("Content-Type: image/png"));
    }

    #[test]
    fn test_build_message_with_no_attachments_or_inline_files() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            attachment: None,
            file_inline: None,
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
            smtp_protocol: None,
        };

        let message = message(&options).unwrap();
        let body = &message.formatted();
        let body = std::str::from_utf8(body).unwrap();

        assert!(body.contains("Hello, world!"));
    }

    #[test]
    fn test_build_message_error_from_invalid_input() {
        let options = Options {
            from: "John Doe".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            attachment: None,
            file_inline: None,
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
            smtp_protocol: None,
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
            attachment: None,
            file_inline: None,
            reply_to: Some("reply@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
            smtp_protocol: None,
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
            attachment: None,
            file_inline: None,
            reply_to: Some("".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
            smtp_protocol: None,
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
            attachment: None,
            file_inline: None,
            reply_to: Some("jane.doe@example.com".to_string()),
            cc: Some(vec!["".to_string()]),
            bcc: Some(vec!["bcc@example.com".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
            smtp_protocol: None,
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
            attachment: None,
            file_inline: None,
            reply_to: Some("jane.doe@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["".to_string()]),
            smtp_server: None,
            smtp_username: None,
            smtp_password: None,
            smtp_protocol: None,
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
            attachment: None,
            file_inline: None,
            reply_to: Some("jane.doe@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["".to_string()]),
            smtp_server: Some("smtp.example.com".to_string()),
            smtp_username: Some("username".to_string()),
            smtp_password: Some("password".to_string()),
            smtp_protocol: Some(Protocol::Smtps),
        };

        let SMTPConfig {
            server,
            username,
            password,
            protocol,
        } = smtp_config(options).unwrap();

        assert_eq!(server, "smtp.example.com");
        assert_eq!(username, "username");
        assert_eq!(password, "password");
        assert_eq!(protocol, Protocol::Smtps);
    }

    #[test]
    fn test_smtp_config_missing_server() {
        let options = Options {
            from: "John Doe <john.doe@example.com>".to_string(),
            to: vec!["jane.doe@example.com".to_string()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
            attachment: None,
            file_inline: None,
            reply_to: Some("jane.doe@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["".to_string()]),
            smtp_server: None,
            smtp_username: Some("username".to_string()),
            smtp_password: Some("password".to_string()),
            smtp_protocol: Some(Protocol::Smtps),
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
            attachment: None,
            file_inline: None,
            reply_to: Some("jane.doe@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["".to_string()]),
            smtp_server: Some("smtp.example.com".to_string()),
            smtp_username: None,
            smtp_password: Some("password".to_string()),
            smtp_protocol: Some(Protocol::Smtps),
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
            attachment: None,
            file_inline: None,
            reply_to: Some("jane.doe@example.com".to_string()),
            cc: Some(vec!["cc@example.com".to_string()]),
            bcc: Some(vec!["".to_string()]),
            smtp_server: Some("smtp.example.com".to_string()),
            smtp_username: Some("username".to_string()),
            smtp_password: None,
            smtp_protocol: None,
        };

        std::env::remove_var("QUERY_SMTP_PASSWORD");

        let result = smtp_config(options);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "SMTP password is required");
    }
}
