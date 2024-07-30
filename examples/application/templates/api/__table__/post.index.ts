import { parse } from "valibot";

import { QUERY_API_QUERY } from "@/config/server/server.constants";
import { {{ tableConstantCase }}_DATABASE } from "@/config/shared/{{ tableLowerCase }}.constants";
import { fetcher } from "@/lib/server/fetcher";
import { handleRequestError } from "@/lib/server/handle-request-error";
import { AUTHORIZATION_REQUEST, CONTENT_TYPE_REQUEST } from "@/lib/server/header";
import { Method } from "@/lib/server/method";
import { ok } from "@/lib/server/responses";
import { queryTokenService } from "@/lib/server/query-token";
import { {{ tablePascalCase }}CreateValidation } from "./{{ tableLowerCase }}.validation";

export async function handleRequest(req: Request): Promise<Response> {
    try {
        const queryToken = await queryTokenService.load("{{ table }}");

        const {{ tableCamelCase }} = await req.json();

        {% for column in columns %}
        const {{ column.columnNameCamelCase }} = {{ tableCamelCase }}.{{ column.columnName }};
        {% endfor %}

        parse({{ tablePascalCase }}CreateValidation, { {% for column in columns %} {% if column.columnLast == false %}{{ column.columnName }}: {{ column.columnNameCamelCase }}, {% else %} {{ column.columnName }}: {{ column.columnNameCamelCase }}{% endif %}{% endfor %} });

        const query = "INSERT INTO {{ tableSnakeCase }} ({% for column in columns %} {{ column.columnName }}{% if column.columnLast == false %}, {% endif %}{% endfor %}) VALUES ({% for column in columns %} :{{ column.columnName }}{% if column.columnLast == false %}, {% endif %}{% endfor %});";
        const params = {
            {% for column in columns %}
            ":{{ column.columnName }}": {{ column.columnNameCamelCase }},
            {% endfor %}
        };

        const response = await fetcher(QUERY_API_QUERY, {
            method: Method.POST,
            body: JSON.stringify({ db_name: {{ tableConstantCase }}_DATABASE, query, params }),
            headers: {
                [AUTHORIZATION_REQUEST]: `Bearer ${queryToken.token}`,
                [CONTENT_TYPE_REQUEST]: "application/json",
            },
        });

        return ok(JSON.stringify(response.json));
    } catch (e) {
        return handleRequestError(e as Error);
    }
}

