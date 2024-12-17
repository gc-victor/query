import { beforeEach, describe, expect, test } from "query:test";
import "./jsx-helpers.js";

describe("__jsxTemplate", () => {
    test("should handle basic static template", () => {
        const result = __jsxTemplate("<div>Hello</div>");
        expect(result.toString()).toBe("<div>Hello</div>");
    });

    test("should handle template with dynamic values", () => {
        const dynamicClass = "active";
        const result = __jsxTemplate(`<div class="${dynamicClass}">Content</div>`);
        expect(result.toString()).toBe('<div class="active">Content</div>');
    });

    test("should handle nested elements with dynamic content", () => {
        const items = ["one", "two"];
        const result = __jsxTemplate(`<div class="test"><ul>${items.map((item) => __jsxTemplate(`<li>${item}</li>`))}</ul></div>`);
        expect(result.toString()).toBe('<div class="test"><ul><li>one</li><li>two</li></ul></div>');
    });
});

describe("__jsxComponent", () => {
    test("should handle component with no props", () => {
        const Component = () => "<div>Basic Component</div>";
        const result = __jsxComponent(Component, []);
        expect(result.toString()).toBe("<div>Basic Component</div>");
    });

    test("should handle component with props", () => {
        const Component = (props) => `<div class="${props.class}">With Props</div>`;
        const result = __jsxComponent(Component, [{ class: "test-class" }]);
        expect(result.toString()).toBe('<div class="test-class">With Props</div>');
    });

    test("should handle component with children", () => {
        const Component = ({ children }) => `<div>${children}</div>`;
        const result = __jsxComponent(Component, [], "<span>Child Content</span>");
        expect(result.toString()).toBe("<div><span>Child Content</span></div>");
    });

    test("should handle component with props and children", () => {
        const Component = ({ children, ...props }) => `<div id="${props.id}">${children}</div>`;
        const result = __jsxComponent(Component, [{ id: "test" }], "<span>Child Content</span>");
        expect(result.toString()).toBe('<div id="test"><span>Child Content</span></div>');
    });
});

describe("__jsxSpread", () => {
    test("should spread object properties into string", () => {
        const props = { class: "test", id: "main", disabled: true };
        const result = __jsxSpread(props);
        expect(result.toString()).toBe('class="test" id="main" disabled');
    });

    test("should handle empty object", () => {
        const props = {};
        const result = __jsxSpread(props);
        expect(result.toString()).toBe("");
    });

    test("should handle boolean and null values", () => {
        const props = {
            visible: true,
            hidden: false,
            empty: null,
            zero: 0,
        };
        const result = __jsxSpread(props);
        expect(result.toString()).toBe('visible zero="0"');
    });

    test("should handle complex jsx with conditions", () => {
        const mockMenuItems = [
            { href: "/home", icon: "home", label: "Home" },
            { href: "/profile", icon: "user", label: "Profile", badge: 3, badgeType: "notification" },
        ];
        const mockUser = { avatar: "/avatar.jpg", name: "John Doe" };
        const mockTheme = "light";
        const mockCurrentPath = "/home";
        const mockStyles = { header: "header-class", link: "link-class", active: "active-class" };

        const Icon = (props) => `<i>${props.name}</i>`;
        const Badge = (props) => `<span type="${props.type}">${props.count}</span>`;

        const result = __jsxTemplate(`
            <div class="${`container ${mockTheme}`}">
                <header class="${mockStyles.header}">
                    <h1>${"Default Title"}</h1>
                    <nav>${mockMenuItems.map((item, index) =>
                        __jsxTemplate(`<a key="${index}" href="${item.href}" class="${`${mockStyles.link} ${mockCurrentPath === item.href ? mockStyles.active : ""}`}">
                            ${item.icon && __jsxComponent(Icon, { name: item.icon })}
                            <span>${item.label}</span>
                            ${item.badge ? __jsxComponent(Badge, { count: item.badge, type: item.badgeType }) : ""}
                        </a>`),
                    )}</nav>
                    ${
                        mockUser
                            ? __jsxTemplate(`<div class="user-menu">
                            <img src="${mockUser.avatar}" alt="User avatar" />
                            <span>${mockUser.name}</span>
                            <button onclick="handleLogout">Logout</button>
                        </div>`)
                            : __jsxTemplate(`<button class="login-button" onclick="handleLogin">Login</button>`)
                    }
                </header>
            </div>
        `);

        const expected =
            '<div class="container light"><header class="header-class"><h1>Default Title</h1><nav><a key="0" href="/home" class="link-class active-class"><i>home</i><span>Home</span></a><a key="1" href="/profile" class="link-class "><i>user</i><span>Profile</span><span type="notification">3</span></a></nav><div class="user-menu"><img src="/avatar.jpg" alt="User avatar" /><span>John Doe</span><button onclick="handleLogout">Logout</button></div></header></div>';

        expect(result.toString().replace(/\s+/g, "")).toBe(expected.replace(/\s+/g, ""));
    });
});
