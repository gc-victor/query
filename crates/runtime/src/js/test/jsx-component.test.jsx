import { describe, expect, test } from "query:test";

const Button = ({ className = "", dark = false, children = "Subimt", ...props }) => {
    const colorSchema = dark ? "dark" : "light";
    const classNames = `${colorSchema}${className ? ` ${className}` : ""}`;
    return (
        <button {...props} class={classNames} type="button">
            {children}
        </button>
    );
};

describe("Button", () => {
    test("renders a self closing button", () => {
        const button = <Button />;
        expect(button).toBe(
            <button class="light" type="button">
                Subimt
            </button>,
        );
    });

    test("should handle prop types correctly", () => {
        const button = <Button />;
        expect(button).toBe(
            <button class="light" type="button">
                Subimt
            </button>,
        );
    });

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
        const button = <Button className="button-class">Click me</Button>;
        expect(button).toBe(
            <button class="light button-class" type="button">
                Click me
            </button>,
        );
    });

    test("renders with spread props", () => {
        const props = {
            disabled: true,
            title: "Button Title",
            onClick: () => {},
        };
        const button = <Button {...props}>Click me</Button>;
        expect(button).toBe(
            <button disabled title="Button Title" onclick="()=>{}" class="light" type="button">
                Click me
            </button>,
        );
    });

    test("renders with empty spread props", () => {
        const props = {};
        const button = <Button{...props}>Click me</Button>;
        expect(button).toBe(
            <button class="light" type="button">
                Click me
            </button>,
        );
    });

    test("renders with empty spread props and attrs", () => {
        const props = {};
        const button = <Button{...props}test="true">Click me</Button>;
        expect(button).toBe(
            <button test="true" class="light" type="button">
                Click me
            </button>,
        );
    });
});
