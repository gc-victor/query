import { minLength, object, {{ columnsListOfUniqueTSTypes }}, uuid } from "valibot";

export const {{ tablePascalCase }}CreateValidation = object({
    {% for column in columns %}
    {% if column.columnType == uuid %}
    {{ column.columnName }}: string([uuid()]),
    {% endif %}{% if column.columnTypeMatchTS == string && column.columnType != uuid %}
    {{ column.columnName }}: string([minLength(1, "Please enter a {{ column.columnName }}.")]),
    {% endif %}{% if column.columnTypeMatchTS == number %}
    {{ column.columnName }}: number("Please enter a {{ column.columnName }}."),
    {% endif %}{% if column.columnTypeMatchTS == Blob %}
    {{ column.columnName }}: blob("Please enter a {{ column.columnName }}."),
    {% endif %}{% if column.columnTypeMatchTS == boolean %}
    {{ column.columnName }}: boolean("Please enter a {{ column.columnName }}."),
    {% endif %}
    {% endfor %}
});

export const {{ tablePascalCase }}UpdateValidation = object({
    uuid: string([uuid()]),
    {% for column in columns %}
    {% if column.columnType == uuid %}
    {{ column.columnName }}: string([uuid()]),
    {% endif %}{% if column.columnTypeMatchTS == string && column.columnType != uuid %}
    {{ column.columnName }}: string([minLength(1, "Please enter a {{ column.columnName }}.")]),
    {% endif %}{% if column.columnTypeMatchTS == number %}
    {{ column.columnName }}: number("Please enter a {{ column.columnName }}."),
    {% endif %}{% if column.columnTypeMatchTS == Blob %}
    {{ column.columnName }}: blob("Please enter a {{ column.columnName }}."),
    {% endif %}{% if column.columnTypeMatchTS == boolean %}
    {{ column.columnName }}: boolean("Please enter a {{ column.columnName }}."),
    {% endif %}
    {% endfor %}
});

export const {{ tablePascalCase }}DeleteValidation = object({
    uuid: string([uuid()])
});
