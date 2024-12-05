export const render = (html: ComponentChildren): string => {
    return `<!DOCTYPE html>${StringHTML(`<html lang="en">${html}</html>`)}`;
};
