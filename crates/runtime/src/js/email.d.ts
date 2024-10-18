type EmailAddress = string;

/**
 * Options for sending an email.
 */
interface EmailOptions {
    /** Sender's email address */
    from: EmailAddress;
    /** Recipient's email address(es) */
    to: EmailAddress | EmailAddress[];
    /** Email subject */
    subject: string;
    /** Email body content */
    body: string;
    /** Reply-to email address */
    replyTo?: EmailAddress;
    /** Carbon copy recipient(s) */
    cc?: EmailAddress | EmailAddress[];
    /** Blind carbon copy recipient(s) */
    bcc?: EmailAddress | EmailAddress[];
    /** SMTP server address */
    smtp_server?: string;
    /** SMTP username */
    smtp_username?: string;
    /** SMTP password */
    smtp_password?: string;
}

/**
 * Interface for an email sending module.
 */
interface Email {
    /**
     * Sends an email with the given options.
     * @param options - The email options including content and SMTP configuration.
     * @throws Will throw an error if the SMTP configuration is invalid.
     * @throws Will throw an error if the email fails to send.
     */
    send(options: EmailOptions): void;
}

export type { EmailAddress, EmailOptions, Email };
