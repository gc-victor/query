import { API_ADMIN_POST_PATH } from "@/config/shared/post.constants";
import { Button } from "@/pages/admin/components/button";
import { Input } from "@/pages/admin/components/input";
import { Legend } from "@/pages/admin/components/legend";
import { Textarea } from "@/pages/admin/components/textarea";
import { ID_FORM_COMPONENT } from "@/pages/admin/utils/constants";

export function PostFormView() {
    return (
        <form is="form-element" id={ID_FORM_COMPONENT} class="h-full pt-6 w-full" method="dialog" data-path={API_ADMIN_POST_PATH}>
            <div class="absolute right-4 text-3xl top-4">
                <Button variant="transparent" type="reset">
                    <span class="block px-3 py-1">
                        <span class="sr-only">Close</span>
                        <span aria-hidden="true">×</span>
                    </span>
                </Button>
            </div>
            <fieldset class="max-h-svh -mb-40 overflow-y-auto pb-40 px-6 space-y-6" tabindex={-1}>
                <Legend>Post</Legend>
                <Input id="title" label="Title" aria-required="true" placeholder="Write a title here..." />
                <Input id="slug" label="Slug" aria-required="true" placeholder="Write a slug here..." />
                <Input
                    id="image"
                    label="Image"
                    type="file"
                    accept="image/*"
                    aria-required="true"
                    placeholder="Write an image url here..."
                />
                <input name="image_url" type="hidden" />
                <Textarea id="content" label="Content" aria-required="true" placeholder="Write a content here..." />
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
