# Email Module

Query provides a simple yet powerful email sending module through the `email` module. This module allows you to send emails with attachments and inline files using either a custom SMTP server or Query's default email service.

## Basic Usage

```javascript
import { email } from "query:email";

await email.send({
    from: "sender@example.com",
    to: "recipient@example.com",
    subject: "Hello from Query!",
    body: "This is a test email."
});
```

## API Reference

### email.send(options)

Sends an email with the specified options.

#### Parameters

`options` object with the following properties:

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| from | string | Yes | The sender's email address |
| to | string \| string[] | Yes | Single recipient or array of recipients |
| subject | string | Yes | Email subject line |
| body | string | Yes | Email body content |
| replyTo | string | No | Reply-to email address |
| cc | string \| string[] | No | Carbon copy recipient(s) |
| bcc | string \| string[] | No | Blind carbon copy recipient(s) |
| attachment | Attachment[] | No | Array of file attachments |
| fileInline | FileInline[] | No | Array of inline files (e.g., embedded images) |
| smtpServer | string | No | Custom SMTP server hostname |
| smtpUsername | string | No | SMTP authentication username |
| smtpPassword | string | No | SMTP authentication password |
| smtpProtocol | "smtps" \| "smtp" | No | SMTP protocol (defaults to "smtps") |

#### Type Definitions

```typescript
type EmailAddress = string;

interface Attachment {
    filename: string;      // Name of the attachment file
    content: Buffer;       // File content as Buffer
    contentType: string;   // MIME type of the file
}

interface FileInline {
    content: Buffer;       // File content as Buffer
    contentType: string;   // MIME type of the file
    contentId: string;     // Unique identifier for referencing in HTML
}

interface EmailResponse {
    code: number;         // Response code from the email server
    message: string;      // Response message from the server
}
```

#### Returns

`Promise<EmailResponse>` that resolves when the email is sent successfully.

#### Errors

Throws an Error if:

- `from` field is missing
- `to` field is missing or empty array
- `subject` field is missing
- `body` field is missing
- SMTP configuration is invalid
- Email fails to send
- Attachment or inline file options are invalid

## Examples

### Basic Email

```javascript
await email.send({
    from: "sender@example.com",
    to: "recipient@example.com",
    subject: "Simple Test",
    body: "This is a basic email test."
});
```

### Email with Attachments

```javascript
await email.send({
    from: "sender@example.com",
    to: "recipient@example.com",
    subject: "Email with Attachments",
    body: "Please find the attached files.",
    attachment: [
        {
            filename: "document.pdf",
            content: pdfContent,
            contentType: "application/pdf"
        },
        {
            filename: "image.jpg",
            content: imageContent,
            contentType: "image/jpeg"
        }
    ]
});
```

### HTML Email with Inline Images

```javascript
await email.send({
    from: "sender@example.com",
    to: "recipient@example.com",
    subject: "Email with Inline Images",
    body: `
        <html>
            <body>
                <h1>Welcome!</h1>
                <p>Here's our logo:</p>
                <img src="cid:logo@company.com" alt="Company Logo" />
            </body>
        </html>
    `,
    fileInline: [
        {
            content: logoContent,
            contentType: "image/png",
            contentId: "logo@company.com"
        }
    ]
});
```

### Multiple Recipients with Attachments and Custom SMTP

```javascript
await email.send({
    from: "sender@example.com",
    to: ["recipient1@example.com", "recipient2@example.com"],
    cc: "manager@example.com",
    subject: "Monthly Report",
    body: "Please find the monthly report attached.",
    attachment: [
        {
            filename: "report.pdf",
            content: reportContent,
            contentType: "application/pdf"
        }
    ],
    smtpServer: "smtp.myserver.com",
    smtpUsername: "username",
    smtpPassword: "password",
    smtpProtocol: "smtps"
});
```
