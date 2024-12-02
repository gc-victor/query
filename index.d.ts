export type * from "./crates/runtime/src/js/database";
export type * from "./crates/runtime/src/js/email";

declare global {
    // NOTE: To avoid editor ts error
    namespace JSX {
        interface Element {
            type: string;
            props: { [key: string]: unknown };
            children: unknown[];
        }

        interface IntrinsicElements {
            [elemName: string]: unknown;
        }
    }
    type ComponentChild = object | string | number | bigint | boolean | null | undefined;
    type ComponentChildren = ComponentChild[] | ComponentChild;
    const StringHTML: (input: string) => string;
}
