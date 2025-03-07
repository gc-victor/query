export interface DocumentationPage {
    content: string;
    description: string;
    markdown: string;
    metadata: Record<string, unknown>;
    navigation: {
        current: {
            title: string;

            url: string;
        };

        next?: {
            title: string;

            url: string;
        } | null;

        previous?: {
            title: string;

            url: string;
        } | null;
    };
    path: string;
    plain_text: string;
    title: string;
}

interface NavLink {
    title: string;
    url: string;
}

interface Current {
    title: string;
    url: string;
}

export interface Navigation {
    previous: NavLink | null;
    next: NavLink | null;
    current: Current;
}

export interface Toc {
    items: GroupOfItems[];
}

interface TocItem {
    group: string;
    title: string;
    url: string;
    level: number;
    children: TocItem[];
}

interface GroupOfItems {
    name: string;
    items: TocItem[];
}
