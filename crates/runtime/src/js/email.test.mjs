import assert from "node:assert";
import { describe, it } from "node:test";
import { email } from "./email.mjs";

describe("email.send", () => {
    let capturedJsonString;

    // Mock the ___email_send function
    global.___email_send = (jsonString) => {
        capturedJsonString = jsonString;
        return "Email sent successfully";
    };

    it("should correctly format and send email data and return success", (t) => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
            replyTo: "reply@example.com",
            cc: "cc@example.com",
            bcc: "bcc@example.com",
            smtp_server: "smtp.example.com",
            smtp_username: "username",
            smtp_password: "password",
        };

        const result = email.send(options);

        const sentData = JSON.parse(capturedJsonString);

        assert.strictEqual(sentData.from, options.from);
        assert.deepStrictEqual(sentData.to, [options.to]);
        assert.strictEqual(sentData.subject, options.subject);
        assert.strictEqual(sentData.body, options.body);
        assert.strictEqual(sentData.reply_to, options.replyTo);
        assert.deepStrictEqual(sentData.cc, [options.cc]);
        assert.deepStrictEqual(sentData.bcc, [options.bcc]);
        assert.strictEqual(sentData.smtp_server, options.smtp_server);
        assert.strictEqual(sentData.smtp_username, options.smtp_username);
        assert.strictEqual(sentData.smtp_password, options.smtp_password);

        assert.deepStrictEqual(result, "Email sent successfully");
    });

    it("should handle missing optional fields and return success", (t) => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
        };

        const result = email.send(options);

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

    it("should handle multiple recipients and return success", (t) => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: ["recipient1@example.com", "recipient2@example.com"],
            subject: "Test Subject",
            body: "Test Body",
        };

        const result = email.send(options);

        const sentData = JSON.parse(capturedJsonString);

        assert.deepStrictEqual(sentData.to, options.to);
        assert.deepStrictEqual(result, "Email sent successfully");
    });

    it("should handle asynchronous operations and return success", async (t) => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Async Test",
            body: "Async Test Body",
        };

        const result = await new Promise((resolve) => {
            const res = email.send(options);
            setTimeout(() => resolve(res), 100);
        });

        const sentData = JSON.parse(capturedJsonString);

        assert.strictEqual(sentData.from, options.from);
        assert.deepStrictEqual(sentData.to, [options.to]);
        assert.strictEqual(sentData.subject, options.subject);
        assert.strictEqual(sentData.body, options.body);
        assert.deepStrictEqual(result, "Email sent successfully");
    });

    // Error handling tests

    it("should throw an error when 'from' field is missing", (t) => {
        const options = {
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
        };

        assert.throws(() => {
            email.send(options);
        }, {
            name: 'Error',
            message: 'From field is required'
        });
    });

    it("should throw an error when 'to' field is missing", (t) => {
        const options = {
            from: "sender@example.com",
            subject: "Test Subject",
            body: "Test Body",
        };

        assert.throws(() => {
            email.send(options);
        }, {
            name: 'Error',
            message: 'To field is required'
        });
    });

    it("should throw an error when 'to' field is an empty array", (t) => {
        const options = {
            from: "sender@example.com",
            to: [],
            subject: "Test Subject",
            body: "Test Body",
        };

        assert.throws(() => {
            email.send(options);
        }, {
            name: 'Error',
            message: 'To field is required'
        });
    });

    it("should throw an error when 'subject' field is missing", (t) => {
        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            body: "Test Body",
        };

        assert.throws(() => {
            email.send(options);
        }, {
            name: 'Error',
            message: 'Subject field is required'
        });
    });

    it("should throw an error when 'body' field is missing", (t) => {
        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
        };

        assert.throws(() => {
            email.send(options);
        }, {
            name: 'Error',
            message: 'Body field is required'
        });
    });

    it("should handle multiple CC recipients and return success", (t) => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
            cc: ["cc1@example.com", "cc2@example.com"],
        };

        const result = email.send(options);

        const sentData = JSON.parse(capturedJsonString);

        assert.deepStrictEqual(sentData.cc, options.cc);
        assert.deepStrictEqual(result, "Email sent successfully");
    });

    it("should handle multiple BCC recipients and return success", (t) => {
        capturedJsonString = null; // Reset before test

        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
            bcc: ["bcc1@example.com", "bcc2@example.com"],
        };

        const result = email.send(options);

        const sentData = JSON.parse(capturedJsonString);

        assert.deepStrictEqual(sentData.bcc, options.bcc);
        assert.deepStrictEqual(result, "Email sent successfully");
    });
});