import { API_ADMIN_{{ tableConstantCase }}_PATH } from "@/config/shared/{{ tableLowerCase }}.constants";
import { Button } from "@/pages/admin/components/button";
import { Input } from "@/pages/admin/components/input";
import { Legend } from "@/pages/admin/components/legend";
import { Textarea } from "@/pages/admin/components/textarea";
import { Switch } from "@/pages/admin/components/switch";
import { ID_FORM_COMPONENT } from "@/pages/admin/utils/constants";

export function {{ tablePascalCase }}FormView() {
    return (
        <form is="form-element" id={ID_FORM_COMPONENT} class="h-full pt-6 w-full" method="dialog" data-path={API_ADMIN_{{ tableConstantCase }}_PATH}>
            <div class="absolute right-4 text-3xl top-4">
                <Button variant="transparent" type="reset">
                    <span class="block px-3 py-1">
                        <span class="sr-only">Close</span>
                        <span aria-hidden="true">×</span>
                    </span>
                </Button>
            </div>
            <fieldset class="max-h-svh -mb-40 overflow-y-auto pb-40 px-6 space-y-6" tabindex={-1}>
                <Legend>{{ tableCapitalCase }}</Legend>
                {% for column in columns  %}
                {% if column.columnType == string || column.columnType == uuid %}
                <Input id="{{ column.columnName }}" label="{{ column.columnNameCapitalCase }}" aria-required="true" placeholder="Write a {{ column.columnName }} here..." />
                {% endif %}{% if column.columnType == text %}
                <Textarea id="{{ column.columnName }}" label="{{ column.columnNameCapitalCase }}" aria-required="true" placeholder="Write a {{ column.columnName }} here..." />
                {% endif %}{% if column.columnType == blob %}
                <Input id="{{ column.columnName }}" label="{{ column.columnNameCapitalCase }}" type="file" aria-required="true" placeholder="Write a {{ column.columnName }} here..." />
                {% endif %}{% if column.columnTypeMatchTS == boolean %}
                <Switch id="{{ column.columnName }}" label="{{ column.columnNameCapitalCase }}" />
                {% endif %}{% if column.columnTypeMatchTS == number %}
                <Input id="{{ column.columnName }}" label="{{ column.columnNameCapitalCase }}" type="number" step="any" aria-required="true" placeholder="Write a {{ column.columnName }} here..." />
                {% endif %}{% if column.columnType == timestamp %}
                <Input id="{{ column.columnName }}" label="{{ column.columnNameCapitalCase }}" type="date" aria-required="true" placeholder="Write a {{ column.columnName }} here..." />
                {% endif %}
                {% endfor %}
            </fieldset>
            <div class="absolute bg-white bottom-0 flex h-20 items-center justify-between px-6 w-full">
                <div class="flex">
                    <Button variant="w-md" type="reset">
                        Cancel
                    </Button>
                    <div class="ml-4">
                        <Button variant="md" type="submit">
                            Submit
                        </Button>
                    </div>
                </div>
                <Button color="red" variant="md" formmethod="delete" type="submit">
                    Delete
                </Button>
            </div>
        </form>
    );
}
