import { handleRequestError } from "./handle-request-error";

type RequestHandler = (req: Request) => Promise<Response>;

export function withRequestErrorHandler(handler: RequestHandler): RequestHandler {
    return async (req: Request): Promise<Response> => {
        try {
            return await handler(req);
        } catch (e) {
            return handleRequestError(e as Error);
        }
    };
}
