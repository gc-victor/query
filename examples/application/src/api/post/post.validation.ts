import { minLength, object, startsWith, string, uuid } from "valibot";

export const PostCreateValidation = object({
    title: string([minLength(1, "Please enter a title.")]),
    content: string([minLength(1, "Please enter a content.")]),
    slug: string([minLength(1, "Please enter a slug."), startsWith("/", "Please enter a slug that starts with a forward slash.")]),
    image_url: string([minLength(1, "Please enter an image_url.")]),
});

export const PostUpdateValidation = object({
    uuid: string([uuid()]),
    title: string([minLength(1, "Please enter a title.")]),
    content: string([minLength(1, "Please enter a content.")]),
    slug: string([minLength(1, "Please enter a slug."), startsWith("/", "Please enter a slug that starts with a forward slash.")]),
    image_url: string([minLength(1, "Please enter an image_url.")]),
});

export const PostDeleteValidation = object({
    uuid: string([uuid()]),
});
