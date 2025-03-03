import type { StatusCode, StatusMessage } from "./status";

export interface ResponseErrorMessage {
    message: StatusMessage | string;
    status: StatusCode | number;
}

export class ResponseError extends Error {
    #error: ResponseErrorMessage;

    constructor(
        public message: string,
        public status: StatusCode | number,
    ) {
        super(message);
        this.#error = { message, status: Number(status) };
    }

    get cause(): ResponseErrorMessage {
        return this.#error;
    }
}
