import { getAssetData } from "@/pages/lib/asset-data";
import { NotFoundError } from "@/pages/lib/types";
import { NotFoundResponse } from "./404";
import { InternalServerErrorResponse } from "./500";
import type { Toc } from "./types";

export function handleError(): ((req: Request, e: Error) => Response | undefined) | undefined {
    return (req, error) => {
        const url = new URL(req.url);
        const toc = getAssetData<Toc>("dist/docs/toc.json");

        if (error instanceof NotFoundError) {
            return NotFoundResponse({ origin: url.origin, toc });
        }

        return InternalServerErrorResponse({ origin: url.origin, toc });
    };
}
