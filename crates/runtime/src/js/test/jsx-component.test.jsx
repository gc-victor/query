import { describe, expect, test } from "query:test";

const Button = ({ className = "", dark = false, children }) => {
    const colorSchema = dark ? "dark" : "light";
    const classNames = `${colorSchema}${className ? ` ${className}` : ""}`;
    return (
        <button class={classNames} type="button">
            {children}
        </button>
    );
};

describe("Button", () => {
    test("renders with children", () => {
        const button = <Button>Click me</Button>;
        expect(button).toBe(
            <button class="light" type="button">
                Click me
            </button>,
        );
    });

    test("renders without dark prop", () => {
        const button = <Button>Click me</Button>;
        expect(button).toBe(
            <button class="light" type="button">
                Click me
            </button>,
        );
    });

    test("renders with dark prop", () => {
        const button = <Button dark={true}>Click me</Button>;
        expect(button).toBe(
            <button class="dark" type="button">
                Click me
            </button>,
        );
    });

    test("renders with boolean attribute", () => {
        const button = (
            <Button className="button-class" dark>
                Click me
            </Button>
        );
        expect(button).toBe(
            <button class="dark button-class" type="button">
                Click me
            </button>,
        );
    });
    
    test("renders with class", () => {
        const button = (
            <Button className="button-class">
                Click me
            </Button>
        );
        expect(button).toBe(
            <button class="light button-class" type="button">
                Click me
            </button>,
        );
    });
});
