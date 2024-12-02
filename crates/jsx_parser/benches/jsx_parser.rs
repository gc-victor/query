use criterion::{criterion_group, criterion_main, Criterion};
use jsx_parser::jsx_parser::Parser;
use jsx_parser::jsx_precompile::jsx_precompile;

fn bench_simple_element(c: &mut Criterion) {
    let input = "<div>Hello World</div>";
    c.bench_function("simple element", |b| {
        b.iter(|| {
            let mut parser = Parser::new(input);
            parser.parse().unwrap()
        })
    });
}

fn bench_element_with_attributes(c: &mut Criterion) {
    let input = r#"<div className="container" id="main" data-test="value">Content</div>"#;
    c.bench_function("element with attributes", |b| {
        b.iter(|| {
            let mut parser = Parser::new(input);
            parser.parse().unwrap()
        })
    });
}

fn bench_nested_elements(c: &mut Criterion) {
    let input = r#"
        <div className="container">1
            <header>
                <h1>Title</h1>
                <nav>
                    <a href="/">Home</a>
                    <a href="/about">About</a>
                </nav>
            </header>
        </div>
    "#;
    c.bench_function("nested elements", |b| {
        b.iter(|| {
            let mut parser = Parser::new(input);
            parser.parse().unwrap()
        })
    });
}

fn bench_expressions(c: &mut Criterion) {
    let input = r#"<div>{count + 1} items remaining</div>"#;
    c.bench_function("expressions", |b| {
        b.iter(|| {
            let mut parser = Parser::new(input);
            parser.parse().unwrap()
        })
    });
}

fn bench_complex_expressions(c: &mut Criterion) {
    let input = r#"
        <div className={`container ${active ? 'active' : ''}`}>
            {items.map(item => (
                <div key={item.id} className={styles.item}>
                    {item.name}
                    <span>{item.count}</span>
                </div>
            ))}
        </div>
    "#;
    c.bench_function("complex expressions", |b| {
        b.iter(|| {
            let mut parser = Parser::new(input);
            parser.parse().unwrap()
        })
    });
}

fn bench_fragment(c: &mut Criterion) {
    let input = r#"
        <>
            <div>First</div>
            <div>Second</div>
            <div>Third</div>
        </>
    "#;
    c.bench_function("fragment", |b| {
        b.iter(|| {
            let mut parser = Parser::new(input);
            parser.parse().unwrap()
        })
    });
}

fn bench_spread_attributes(c: &mut Criterion) {
    let input = r#"<Component {...props} id="main" className={styles.component} />"#;
    c.bench_function("spread attributes", |b| {
        b.iter(|| {
            let mut parser = Parser::new(input);
            parser.parse().unwrap()
        })
    });
}

fn bench_conditional_rendering(c: &mut Criterion) {
    let input = r#"
        <div>
            {loading ? (
                <Spinner size="large" />
            ) : (
                <Content items={items} />
            )}
        </div>
    "#;
    c.bench_function("conditional rendering", |b| {
        b.iter(|| {
            let mut parser = Parser::new(input);
            parser.parse().unwrap()
        })
    });
}

fn bench_large_component(c: &mut Criterion) {
    let input = r#"
        <div className={`container ${theme}`}>
            <header className={styles.header}>
                <h1>{title || "Default Title"}</h1>
                <nav>
                    {menuItems.map((item, index) => (
                        <a 
                            key={index}
                            href={item.href}
                            className={`${styles.link} ${currentPath === item.href ? styles.active : ''}`}
                        >
                            {item.icon && <Icon name={item.icon} />}
                            <span>{item.label}</span>
                            {item.badge && (
                                <Badge count={item.badge} type={item.badgeType} />
                            )}
                        </a>
                    ))}
                </nav>
                {user ? (
                    <div className={styles.userMenu}>
                        <img src={user.avatar} alt="User avatar" />
                        <span>{user.name}</span>
                        <button onClick={handleLogout}>Logout</button>
                    </div>
                ) : (
                    <button className={styles.loginButton} onClick={handleLogin}>
                        Login
                    </button>
                )}
            </header>
            <main className={styles.main}>
                {loading ? (
                    <div className={styles.loader}>
                        <Spinner size="large" color={theme === 'dark' ? 'white' : 'black'} />
                    </div>
                ) : error ? (
                    <ErrorMessage message={error} onRetry={handleRetry} />
                ) : (
                    <>{children}</>
                )}
            </main>
            <footer className={styles.footer}>
                <p>&copy; {currentYear} My Application</p>
            </footer>
        </div>
    "#;
    c.bench_function("large component", |b| {
        b.iter(|| {
            let mut parser = Parser::new(input);
            parser.parse().unwrap()
        })
    });
}

fn bench_many_siblings(c: &mut Criterion) {
    let input = r#"
        <div>
            <div>1</div>
            <div>2</div>
            <div>3</div>
            <div>4</div>
            <div>5</div>
            <div>6</div>
            <div>7</div>
            <div>8</div>
            <div>9</div>
            <div>10</div>
            <div>11</div>
            <div>12</div>
            <div>13</div>
            <div>14</div>
            <div>15</div>
            <div>16</div>
            <div>17</div>
            <div>18</div>
            <div>19</div>
            <div>20</div>
        </div>
    "#;
    c.bench_function("many siblings", |b| {
        b.iter(|| {
            let mut parser = Parser::new(input);
            parser.parse().unwrap()
        })
    });
}

fn bench_jsx_precompile(c: &mut Criterion) {
    let input = r#"
        <div className={`container ${theme}`}>
            <header className={styles.header}>
                <h1>{title || "Default Title"}</h1>
                <nav>
                    {menuItems.map((item, index) => (
                        <a 
                            key={index}
                            href={item.href}
                            className={`${styles.link} ${currentPath === item.href ? styles.active : ''}`}
                        >
                            {item.icon && <Icon name={item.icon} />}
                            <span>{item.label}</span>
                            {item.badge && (
                                <Badge count={item.badge} type={item.badgeType} />
                            )}
                        </a>
                    ))}
                </nav>
                {user ? (
                    <div className={styles.userMenu}>
                        <img src={user.avatar} alt="User avatar" />
                        <span>{user.name}</span>
                        <button onClick={handleLogout}>Logout</button>
                    </div>
                ) : (
                    <button className={styles.loginButton} onClick={handleLogin}>
                        Login
                    </button>
                )}
            </header>
            <main className={styles.main}>
                {loading ? (
                    <div className={styles.loader}>
                        <Spinner size="large" color={theme === 'dark' ? 'white' : 'black'} />
                    </div>
                ) : error ? (
                    <ErrorMessage message={error} onRetry={handleRetry} />
                ) : (
                    <>{children}</>
                )}
            </main>
            <footer className={styles.footer}>
                <p>&copy; {currentYear} My Application</p>
            </footer>
        </div>
    "#;
    c.bench_function("jsx precompile large component", |b| {
        b.iter(|| {
            jsx_precompile(input).unwrap()
        })
    });
}

criterion_group!(benches, 
    bench_simple_element,
    bench_element_with_attributes,
    bench_nested_elements,
    bench_expressions,
    bench_complex_expressions,
    bench_fragment,
    bench_spread_attributes,
    bench_conditional_rendering,
    bench_large_component,
    bench_many_siblings,
    bench_jsx_precompile
);

criterion_main!(benches);