import { describe, expect, test } from "query:test";
import { email } from "./email.js";

describe("Basic email functionality", () => {
    test("should correctly format and send email data and return success", async () => {
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

        let capturedJsonString = null;
        global.___send_email = (jsonString) => {
            capturedJsonString = jsonString;
            return "Email sent successfully";
        };

        const result = await email.send(options);
        const sentData = JSON.parse(capturedJsonString);

        expect(sentData.from).toBe(options.from);
        expect(sentData.to).toDeepEqual([options.to]);
        expect(sentData.subject).toBe(options.subject);
        expect(sentData.body).toBe(options.body);
        expect(sentData.reply_to).toBe(options.replyTo);
        expect(sentData.cc).toDeepEqual([options.cc]);
        expect(sentData.bcc).toDeepEqual([options.bcc]);
        expect(sentData.smtp_server).toBe(options.smtpServer);
        expect(sentData.smtp_username).toBe(options.smtpUsername);
        expect(sentData.smtp_password).toBe(options.smtpPassword);

        expect(result).toBe("Email sent successfully");
    });

    test("should handle missing optional fields and return success", async () => {
        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
        };

        let capturedJsonString = null;
        global.___send_email = (jsonString) => {
            capturedJsonString = jsonString;
            return "Email sent successfully";
        };

        const result = await email.send(options);
        const sentData = JSON.parse(capturedJsonString);

        expect(sentData.from).toBe(options.from);
        expect(sentData.to).toDeepEqual([options.to]);
        expect(sentData.subject).toBe(options.subject);
        expect(sentData.body).toBe(options.body);
        expect(sentData.reply_to).toBe(undefined);
        expect(sentData.cc).toBe(undefined);
        expect(sentData.bcc).toBe(undefined);
        expect(sentData.smtp_server).toBe(undefined);
        expect(sentData.smtp_username).toBe(undefined);
        expect(sentData.smtp_password).toBe(undefined);

        expect(result).toBe("Email sent successfully");
    });
});

describe("Recipients handling", () => {
    let capturedJsonString;

    test("should handle multiple recipients and return success", async () => {
        const options = {
            from: "sender@example.com",
            to: ["recipient1@example.com", "recipient2@example.com"],
            subject: "Test Subject",
            body: "Test Body",
        };

        let capturedJsonString = null;
        global.___send_email = (jsonString) => {
            capturedJsonString = jsonString;
            return "Email sent successfully";
        };

        const result = await email.send(options);
        const sentData = JSON.parse(capturedJsonString);

        expect(sentData.to).toDeepEqual(options.to);
        expect(result).toBe("Email sent successfully");
    });

    test("should handle multiple CC recipients and return success", async () => {
        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
            cc: ["cc1@example.com", "cc2@example.com"],
        };

        let capturedJsonString = null;
        global.___send_email = (jsonString) => {
            capturedJsonString = jsonString;
            return "Email sent successfully";
        };

        const result = await email.send(options);
        const sentData = JSON.parse(capturedJsonString);

        expect(sentData.cc).toDeepEqual(options.cc);
        expect(result).toBe("Email sent successfully");
    });

    test("should handle multiple BCC recipients and return success", async () => {
        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
            bcc: ["bcc1@example.com", "bcc2@example.com"],
        };

        let capturedJsonString = null;
        global.___send_email = (jsonString) => {
            capturedJsonString = jsonString;
            return "Email sent successfully";
        };

        const result = await email.send(options);
        const sentData = JSON.parse(capturedJsonString);

        expect(sentData.bcc).toDeepEqual(options.bcc);
        expect(result).toBe("Email sent successfully");
    });
});

describe("Required field validation", () => {
    let capturedJsonString;

    test("should throw an error when 'from' field is missing", async () => {
        const options = {
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
        };

        await expect(async () => {
            await email.send(options);
        }).toThrow("From field is required");
    });

    test("should throw an error when 'to' field is missing", async () => {
        const options = {
            from: "sender@example.com",
            subject: "Test Subject",
            body: "Test Body",
        };

        await expect(async () => {
            await email.send(options);
        }).toThrow("To field is required");
    });

    test("should throw an error when 'to' field is an empty array", async () => {
        const options = {
            from: "sender@example.com",
            to: [],
            subject: "Test Subject",
            body: "Test Body",
        };

        await expect(async () => {
            await email.send(options);
        }).toThrow("To field is required");
    });

    test("should throw an error when 'subject' field is missing", async () => {
        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            body: "Test Body",
        };

        await expect(async () => {
            await email.send(options);
        }).toThrow("Subject field is required");
    });

    test("should throw an error when 'body' field is missing", async () => {
        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
        };

        await expect(async () => {
            await email.send(options);
        }).toThrow("Body field is required");
    });
});

describe("Optional fields handling", () => {
    test("should handle missing 'replyTo' field and return success", async () => {
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

        let capturedJsonString = null;
        global.___send_email = (jsonString) => {
            capturedJsonString = jsonString;
            return "Email sent successfully";
        };

        const result = await email.send(options);
        const sentData = JSON.parse(capturedJsonString);

        expect(sentData.reply_to).toBe(undefined);
        expect(result).toBe("Email sent successfully");
    });

    test("should handle missing 'cc' field and return success", async () => {
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

        let capturedJsonString = null;
        global.___send_email = (jsonString) => {
            capturedJsonString = jsonString;
            return "Email sent successfully";
        };

        const result = await email.send(options);
        const sentData = JSON.parse(capturedJsonString);

        expect(sentData.cc).toBe(undefined);
        expect(result).toBe("Email sent successfully");
    });

    test("should handle missing 'bcc' field and return success", async () => {
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

        let capturedJsonString = null;
        global.___send_email = (jsonString) => {
            capturedJsonString = jsonString;
            return "Email sent successfully";
        };

        const result = await email.send(options);
        const sentData = JSON.parse(capturedJsonString);

        expect(sentData.bcc).toBe(undefined);
        expect(result).toBe("Email sent successfully");
    });

    test("should handle missing 'smtpServer' field and return success", async () => {
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

        let capturedJsonString = null;
        global.___send_email = (jsonString) => {
            capturedJsonString = jsonString;
            return "Email sent successfully";
        };

        const result = await email.send(options);
        const sentData = JSON.parse(capturedJsonString);

        expect(sentData.smtp_server).toBe(undefined);
        expect(result).toBe("Email sent successfully");
    });

    test("should handle missing 'smtpUsername' field and return success", async () => {
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

        let capturedJsonString = null;
        global.___send_email = (jsonString) => {
            capturedJsonString = jsonString;
            return "Email sent successfully";
        };

        const result = await email.send(options);
        const sentData = JSON.parse(capturedJsonString);

        expect(sentData.smtp_username).toBe(undefined);
        expect(result).toBe("Email sent successfully");
    });

    test("should handle missing 'smtpPassword' field and return success", async () => {
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

        let capturedJsonString = null;
        global.___send_email = (jsonString) => {
            capturedJsonString = jsonString;
            return "Email sent successfully";
        };

        const result = await email.send(options);
        const sentData = JSON.parse(capturedJsonString);

        expect(sentData.smtp_password).toBe(undefined);
        expect(result).toBe("Email sent successfully");
    });
});

describe("Attachment handling", () => {
    test("should handle attachment field and return success", async () => {
        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
            replyTo: "reply@example.com",
            cc: "cc@example.com",
            bcc: "bcc@example.com",
            attachment: [
                {
                    filename: "file.txt",
                    content: "file content",
                    contentType: "text/plain",
                },
            ],
        };

        let capturedJsonString = null;
        global.___send_email = (jsonString) => {
            capturedJsonString = jsonString;
            return "Email sent successfully";
        };

        await email.send(options);
        const sentData = JSON.parse(capturedJsonString);

        expect(sentData.attachment).toDeepEqual(options.attachment);
    });

    test("should handle fileInline field and return success", async () => {
        const options = {
            from: "sender@example.com",
            to: "recipient@example.com",
            subject: "Test Subject",
            body: "Test Body",
            replyTo: "reply@example.com",
            cc: "cc@example.com",
            bcc: "bcc@example.com",
            fileInline: [
                {
                    content: "file content",
                    contentType: "text/plain",
                    contentId: "123",
                },
            ],
        };

        let capturedJsonString = null;
        global.___send_email = (jsonString) => {
            capturedJsonString = jsonString;
            return "Email sent successfully";
        };

        await email.send(options);
        const sentData = JSON.parse(capturedJsonString);

        expect(sentData.file_inline).toDeepEqual(options.fileInline);
    });
});
