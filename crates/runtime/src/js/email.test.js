import assert from "node:assert";
import { describe, it } from "node:test";
import { email } from "./email.js";

describe("email.send", () => {
    let capturedJsonString;

    // Mock the ___send_email function
    global.___send_email = (jsonString) => {
        capturedJsonString = jsonString;
        return "Email sent successfully";
    };

    it("should correctly format and send email data and return success", async () => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
            replyTo: "reply@example.com",
            cc: "cc@example.com",
            bcc: "bcc@example.com",
            smtpServer: "smtp.example.com",
            smtpUsername: "username",
            smtpPassword: "password",
        };

        const result = await email.send(options);

        const sentData = JSON.parse(capturedJsonString);

        assert.strictEqual(sentData.from, options.from);
        assert.deepStrictEqual(sentData.to, [options.to]);
        assert.strictEqual(sentData.subject, options.subject);
        assert.strictEqual(sentData.body, options.body);
        assert.strictEqual(sentData.reply_to, options.replyTo);
        assert.deepStrictEqual(sentData.cc, [options.cc]);
        assert.deepStrictEqual(sentData.bcc, [options.bcc]);
        assert.strictEqual(sentData.smtp_server, options.smtpServer);
        assert.strictEqual(sentData.smtp_username, options.smtpUsername);
        assert.strictEqual(sentData.smtp_password, options.smtpPassword);

        assert.deepStrictEqual(result, "Email sent successfully");
    });

    it("should handle missing optional fields and return success", async () => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
        };

        const result = await email.send(options);

        const sentData = JSON.parse(capturedJsonString);

        assert.strictEqual(sentData.from, options.from);
        assert.deepStrictEqual(sentData.to, [options.to]);
        assert.strictEqual(sentData.subject, options.subject);
        assert.strictEqual(sentData.body, options.body);
        assert.strictEqual(sentData.reply_to, undefined);
        assert.strictEqual(sentData.cc, undefined);
        assert.strictEqual(sentData.bcc, undefined);
        assert.strictEqual(sentData.smtp_server, undefined);
        assert.strictEqual(sentData.smtp_username, undefined);
        assert.strictEqual(sentData.smtp_password, undefined);

        assert.deepStrictEqual(result, "Email sent successfully");
    });

    it("should handle multiple recipients and return success", async () => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: ["recipient1@example.com", "recipient2@example.com"],
            subject: "Test Subject",
            body: "Test Body",
        };

        const result = await email.send(options);

        const sentData = JSON.parse(capturedJsonString);

        assert.deepStrictEqual(sentData.to, options.to);
        assert.deepStrictEqual(result, "Email sent successfully");
    });

    it("should throw an error when 'from' field is missing", async () => {
        const options = {
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
        };

        await assert.rejects(
            async () => {
                await email.send(options);
            },
            {
                name: "Error",
                message: "From field is required",
            },
        );
    });

    it("should throw an error when 'to' field is missing", async () => {
        const options = {
            from: "sender@example.com",
            subject: "Test Subject",
            body: "Test Body",
        };

        await assert.rejects(
            async () => {
                await email.send(options);
            },
            {
                name: "Error",
                message: "To field is required",
            },
        );
    });

    it("should throw an error when 'to' field is an empty array", async () => {
        const options = {
            from: "sender@example.com",
            to: [],
            subject: "Test Subject",
            body: "Test Body",
        };

        await assert.rejects(
            async () => {
                await email.send(options);
            },
            {
                name: "Error",
                message: "To field is required",
            },
        );
    });

    it("should throw an error when 'subject' field is missing", async () => {
        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            body: "Test Body",
        };

        await assert.rejects(
            async () => {
                await email.send(options);
            },
            {
                name: "Error",
                message: "Subject field is required",
            },
        );
    });

    it("should throw an error when 'body' field is missing", async () => {
        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
        };

        await assert.rejects(
            async () => {
                await email.send(options);
            },
            {
                name: "Error",
                message: "Body field is required",
            },
        );
    });

    it("should handle multiple CC recipients and return success", async () => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
            cc: ["cc1@example.com", "cc2@example.com"],
        };

        const result = await email.send(options);

        const sentData = JSON.parse(capturedJsonString);

        assert.deepStrictEqual(sentData.cc, options.cc);
        assert.deepStrictEqual(result, "Email sent successfully");
    });

    it("should handle multiple BCC recipients and return success", async () => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
            bcc: ["bcc1@example.com", "bcc2@example.com"],
        };

        const result = await email.send(options);

        const sentData = JSON.parse(capturedJsonString);

        assert.deepStrictEqual(sentData.bcc, options.bcc);
        assert.deepStrictEqual(result, "Email sent successfully");
    });

    it("should handle missing 'replyTo' field and return success", async () => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
            cc: "cc@example.com",
            bcc: "bcc@example.com",
            smtpServer: "smtp.example.com",
            smtpUsername: "username",
            smtpPassword: "password",
        };

        const result = await email.send(options);

        const sentData = JSON.parse(capturedJsonString);

        assert.strictEqual(sentData.reply_to, undefined);
        assert.deepStrictEqual(result, "Email sent successfully");
    });

    it("should handle missing 'cc' field and return success", async () => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
            replyTo: "reply@example.com",
            bcc: "bcc@example.com",
            smtpServer: "smtp.example.com",
            smtpUsername: "username",
            smtpPassword: "password",
        };

        const result = await email.send(options);

        const sentData = JSON.parse(capturedJsonString);

        assert.strictEqual(sentData.cc, undefined);
        assert.deepStrictEqual(result, "Email sent successfully");
    });

    it("should handle missing 'bcc' field and return success", async () => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
            replyTo: "reply@example.com",
            cc: "cc@example.com",
            smtpServer: "smtp.example.com",
            smtpUsername: "username",
            smtpPassword: "password",
        };

        const result = await email.send(options);

        const sentData = JSON.parse(capturedJsonString);

        assert.strictEqual(sentData.bcc, undefined);
        assert.deepStrictEqual(result, "Email sent successfully");
    });

    it("should handle missing 'smtpServer' field and return success", async () => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
            replyTo: "reply@example.com",
            cc: "cc@example.com",
            bcc: "bcc@example.com",
            smtpUsername: "username",
            smtpPassword: "password",
        };

        const result = await email.send(options);

        const sentData = JSON.parse(capturedJsonString);

        assert.strictEqual(sentData.smtp_server, undefined);
        assert.deepStrictEqual(result, "Email sent successfully");
    });

    it("should handle missing 'smtpUsername' field and return success", async () => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
            replyTo: "reply@example.com",
            cc: "cc@example.com",
            bcc: "bcc@example.com",
            smtpServer: "smtp.example.com",
            smtpPassword: "password",
        };

        const result = await email.send(options);

        const sentData = JSON.parse(capturedJsonString);

        assert.strictEqual(sentData.smtp_username, undefined);
        assert.deepStrictEqual(result, "Email sent successfully");
    });

    it("should handle missing 'smtpPassword' field and return success", async () => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
            replyTo: "reply@example.com",
            cc: "cc@example.com",
            bcc: "bcc@example.com",
            smtpServer: "smtp.example.com",
            smtpUsername: "username",
        };

        const result = await email.send(options);

        const sentData = JSON.parse(capturedJsonString);

        assert.strictEqual(sentData.smtp_password, undefined);
        assert.deepStrictEqual(result, "Email sent successfully");
    });
});
