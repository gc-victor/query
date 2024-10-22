const emailFunction = `
import { email } from 'query:email';

globalThis.___handleRequest = async () => {

    try {
        const options = {
            from: process.env.EMAIL_FROM,
            to: process.env.EMAIL_TO,
            subject: "Test email",
            body: "<p>This is a test email</p>",
        };

        const response = await email.send(options);

        console.log("Email sent", response);

        return new Response("OK", { status: 200 });
    } catch (error) {
        console.error(error);
        return new Response("Error", { status: 500 });
    }
}`;

console.log(`[${Array.from(new TextEncoder("utf-8").encode(emailFunction)).toString()}]`);
