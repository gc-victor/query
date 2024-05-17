export interface {{ tablePascalCase }}Type {
    {% for column in columns %}
    {{ column.columnNameCamelCase }}: {{ column.columnTypeMatchTS }};
    {% endfor %}
    datetime: string;
    createdAt: string;
}

export function {{ tablePascalCase }}({ {% for column in columns %} {{ column.columnNameCamelCase }},{% endfor %} datetime, createdAt }: {{ tablePascalCase }}Type) {
    return (
        <div class="mt-8">
            {% for column in columns %}
            {% if column.columnTypeMatchTS == boolean %}
            <p><span class="font-bold">{{ column.columnNameCapitalCase }}</span>: { {{ column.columnNameCamelCase }} ? "on" : "off" }</p>
            {% endif %}{% if column.columnTypeMatchTS == number %}
            <p><span class="font-bold">{{ column.columnNameCapitalCase }}</span>: { String({{ column.columnNameCamelCase }}) }</p>
            {% endif %}{% if column.columnType == timestamp %}
            <p><span class="font-bold">{{ column.columnNameCapitalCase }}</span>: {date(Number({{ column.columnNameCamelCase }}))} | <span>{time(Number({{ column.columnNameCamelCase }}))}</span></p>
            {% endif %}{% if column.columnTypeMatchTS != boolean && column.columnTypeMatchTS != number && column.columnType != timestamp %}
            <p><span class="font-bold">{{ column.columnNameCapitalCase }}</span>: { {{ column.columnNameCamelCase }} }</p>
            {% endif %}
            {% endfor %}
        </div>
    );
}

function date(timestamp: number) {
    const lang = typeof navigator !== "undefined" && navigator.language;
    return new Date(timestamp * 1000).toLocaleDateString(lang || "en-US", { day: "2-digit", month: "2-digit", year: "numeric" });
}

function time(timestamp: number) {
    const date = new Date(timestamp * 1000);
    const hours = String(date.getUTCHours()).padStart(2, "0");
    const minutes = String(date.getUTCMinutes()).padStart(2, "0");
    const seconds = String(date.getUTCSeconds()).padStart(2, "0");

    return `${hours}:${minutes}:${seconds} UTC`;
}
