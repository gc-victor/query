import type { StatusCode, StatusMessage } from "./status";

export interface ResponseErrorMessage {
    message: StatusMessage;
    status: StatusCode;
}

export class ResponseError extends Error {
    #error: ResponseErrorMessage;

    constructor(error: string) {
        super(error);
        this.#error = JSON.parse(error);
    }

    get cause(): ResponseErrorMessage {
        return this.#error;
    }
}

export type SessionError = Error;
