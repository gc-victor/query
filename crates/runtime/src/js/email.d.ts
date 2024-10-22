type EmailAddress = string;

interface Attachment {
    /** Attachment file name */
    filename: string;
    /** Attachment content */
    content: Buffer;
    /** Attachment content type */
    contentType: string;
}

interface FileInline {
    /** File inline content */
    content: Buffer;
    /** File inline content type */
    contentType: string;
    /** File inline content id */
    contentId: string;
}

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
    /** Attachment */
    attachment?: Attachment[];
    /** File inline */
    fileInline?: FileInline[];
    /** SMTP server address */
    smtpServer?: string;
    /** SMTP username */
    smtpUsername?: string;
    /** SMTP password */
    smtpPassword?: string;
    /** SMTP protocol */
    smtpProtocol?: "smtps" | "smtp"; // By default, smtps is used
}

interface EmailResponse {
    code: number;
    message: string;
}

/**
 * Interface for an email sending module.
 */
interface Email {
    /**
     * Sends an email with the given options.
     * @param options - The email options including content and SMTP configuration.
     * @returns A promise that resolves with the email response
     * @throws Will throw an error if any of the options is invalid.
     * @throws Will throw an error if the SMTP configuration is invalid.
     * @throws Will throw an error if the email fails to send.
     */
    send(options: EmailOptions): Promise<EmailResponse>;
}

export type { EmailAddress, EmailOptions, Email };
