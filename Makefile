ARGUMENTS = $(filter-out $@,$(MAKECMDGOALS))

export HURL_prev_month := $(shell date +%s%3N -d '-1 month')
export HURL_next_month := $(shell date +%s%3N -d '+1 month')
export HURL_next_year := $(shell date +%s%3N -d '+1 year')

# Clean

clean:
	rm -rf Cargo.lock | cargo clean

clean-dbs:
	rm -rf .dbs && rm -rf .tests

clean-hurl-dbs:
	rm -rf .dbs/hurl*.sql

# CLI

install-esbuild:
	curl -fsSL https://esbuild.github.io/dl/latest | sh

cli:
	cp -r esbuild target/debug/
	target/debug/query $(ARGUMENTS)

# Coverage

install-llvm-cov:
	cargo install cargo-llvm-cov

coverage-clean:
	cargo llvm-cov clean --workspace

coverage:
	cargo llvm-cov --workspace --exclude query nextest

coverage-watch:
	cargo watch -c -s "make coverage-lcov"

coverage-report:
	cargo llvm-cov report

coverage-html:
	cargo llvm-cov --workspace --exclude query --html nextest

coverage-lcov: coverage-clean
	cargo llvm-cov --workspace --exclude query --lcov --output-path target/llvm-cov-target/lcov.info nextest

# Dist

install-cargo-dist:
	cargo install cargo-dist --locked

dist-plan:
	cargo dist plan

# Docs

docs:
	cargo doc --package=server --open

# Format

fmt: fmt-server

fmt-server:
	cargo fmt --all --package=server -- --check

fmt-cli:
	cargo fmt --all --package=query -- --check

# Hurl

install-hurl:
	cargo install hurl

hurl: clean-hurl-dbs
	hurl --verbose --continue-on-error --variables-file hurl/.env $(ARGUMENTS)

hurl-test: clean-hurl-dbs
	hurl --test --continue-on-error --variables-file hurl/.env $(ARGUMENTS)

hurl-test-all: clean-hurl-dbs
	hurl --test --continue-on-error --variables-file hurl/.env hurl/**/*.hurl hurl/**/**/*.hurl

# Lint

lint:
	cargo clippy --all-targets --all-features --workspace

# Release

build-server:
	cargo build --package=server --release

build-cli:
	cargo build --package=query --profile dist

# Run

run:
	RUST_LOG=info cargo run --package=server -q | bunyan

run-cli:
	RUST_LOG=info cargo run --package=query

run-release:
	RUST_LOG=info cargo run --package=server --release -q | bunyan

run-cli-release:
	RUST_LOG=info cargo run --package=query --profile dist

dev-cli:
	cargo watch --ignore .dbs --shell "make run-cli -s"

dev:
	cargo watch --ignore .dbs --shell "make run -s"

# Test

install-nextest:
	cargo install cargo-nextest --locked

test:
	cargo test -- --test-threads=1

nextest:
	cargo nextest run

nextest-match:
	cargo nextest run --filter-expr 'test($(ARGUMENTS))'

test-watch:
	cargo watch -c -s "make test"

# catch anything and do nothing
%:
	@: