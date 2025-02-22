import { describe, expect, test } from "query:test";

describe("jsx transformation", () => {
    test("should transform standard JSX", () => {
        const Element = () => <div>Hello World</div>;
        const result = <Element />;
        expect(result.toString()).toBe(StringHTML("<div>Hello World</div>"));
    });

    test("should handle JSX with props", () => {
        const Element = ({ className, id }) => (
            <div className={className} id={id}>
                With Props
            </div>
        );
        const result = <Element className="test" id="main" />;
        expect(result.toString()).toBe(StringHTML('<div class="test" id="main">With Props</div>'));
    });

    test("should handle nested JSX components passing children and props", () => {
        const Child = ({ children, ...props }) => <div><p>{ props.attr }</p>{children}</div>;
        const Parent = (props) => (
            <Child {...props}>
                <p>Parent Content</p>
                <p>Another Content</p>
            </Child>
        );
        const GrandParent = () => (
            <Parent attr="Test">
                <p>GrandParent Content</p>
            </Parent>
        );
        const result = <GrandParent />;
        expect(result.toString()).toBe(StringHTML("<div><p>Test</p><p>Parent Content</p> <p>Another Content</p></div>"));
    });

    test("should handle conditional rendering in JSX", () => {
        const isVisible = true;
        const Element = () => <div>{isVisible ? <span>Visible Content</span> : null}</div>;
        const result = <Element />;
        expect(result.toString()).toBe(StringHTML("<div><span>Visible Content</span></div>"));
    });

    test("should handle JSX list rendering", () => {
        const items = ["one", "two", "three"];
        const List = () => (
            <ul>
                {items.map((item) => (
                    // biome-ignore lint/correctness/useJsxKeyInIterable: <explanation>
                    <li>{item}</li>
                ))}
            </ul>
        );
        const result = <List />;
        expect(result.toString()).toBe(StringHTML("<ul><li>one</li><li>two</li><li>three</li></ul>"));
    });

    test("should handle complex JSX with events", () => {
        const Button = ({ onClick, children }) => (
            <button onClick={onClick} className="btn" type="button">
                {children}
            </button>
        );
        const result = <Button onClick="handleClick">Click Me</Button>;
        expect(result.toString()).toBe(StringHTML('<button onclick="handleClick" class="btn" type="button">Click Me</button>'));
    });

    test("should handle JSX fragments", () => {
        const Fragment = () => (
            <>
                <div>First</div>
                <div>Second</div>
            </>
        );
        const result = <Fragment />;
        expect(result.toString()).toBe(StringHTML("<div>First</div> <div>Second</div>"));
    });

    test("should handle self-closing JSX tags", () => {
        const Element = () => <input type="text" className="input" />;
        const result = <Element />;
        expect(result.toString()).toBe(StringHTML('<input type="text" class="input"/>'));
    });
});
