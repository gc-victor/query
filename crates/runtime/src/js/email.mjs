export const email = {
    send(options) {
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
            smtp_server: options.smtp_server,
            smtp_username: options.smtp_username,
            smtp_password: options.smtp_password,
        };

        return ___email_send(JSON.stringify(emailData));
    },
};
