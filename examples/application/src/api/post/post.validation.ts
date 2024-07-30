import { object, string, uuid, minLength, startsWith, pipe } from 'valibot';

export const PostCreateValidation = object({
    title: pipe(string(), minLength(1, "Please enter a title.")),
    content: pipe(string(), minLength(1, "Please enter a content.")),
    slug: pipe(string(), minLength(1, "Please enter a slug."), startsWith("/", "Please enter a slug that starts with a forward slash.")),
    image_url: pipe(string(), minLength(1, "Please enter an image_url.")),
});

export const PostUpdateValidation = object({
    uuid: pipe(string(), uuid()),
    title: pipe(string(), minLength(1, "Please enter a title.")),
    content: pipe(string(), minLength(1, "Please enter a content.")),
    slug: pipe(string(), minLength(1, "Please enter a slug."), startsWith("/", "Please enter a slug that starts with a forward slash.")),
    image_url: pipe(string(), minLength(1, "Please enter an image_url.")),
});

export const PostDeleteValidation = object({
    uuid: pipe(string(), uuid()),
});
