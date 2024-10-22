export const email = {
    async send(options) {
        if (!options.from) {
            throw new Error("From field is required");
        }
        if (!options.to || (Array.isArray(options.to) && options.to.length === 0)) {
            throw new Error("To field is required");
        }
        if (!options.subject) {
            throw new Error("Subject field is required");
        }
        if (!options.body) {
            throw new Error("Body field is required");
        }

        const emailData = {
            from: options.from,
            to: typeof options.to === "string" ? [options.to] : options.to,
            subject: options.subject,
            body: options.body,
            reply_to: options.replyTo,
            cc: typeof options.cc === "string" ? [options.cc] : options.cc,
            bcc: typeof options.bcc === "string" ? [options.bcc] : options.bcc,
            attachment: options.attachment,
            // The image `attachment` will display inline into the email. E.g. <img src="cid:123">
            file_inline: options.fileInline,
            smtp_server: options.smtpServer,
            smtp_username: options.smtpUsername,
            smtp_password: options.smtpPassword,
            smtp_protocol: options.smtpProtocol,
        };

        return await ___send_email(JSON.stringify(emailData));
    },
};
