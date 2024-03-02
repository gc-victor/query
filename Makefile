ARGUMENTS = $(filter-out $@,$(MAKECMDGOALS))
GET_ARGUMENT = $(strip $(call word,$(1),$(ARGUMENTS)))

# Hurl variables

export HURL_prev_month := $(shell date +%s%3N -d '-1 month')
export HURL_next_month := $(shell date +%s%3N -d '+1 month')
export HURL_next_year := $(shell date +%s%3N -d '+1 year')
HURL_user_token := $(shell curl --silent --header \
	'Content-Type: application/json' \
	--data '{"email":"admin","password":"admin"}' \
	'http://localhost:3000/_/user/token/value' | jq -r '.data[0].token')
HURL_user_token := "Bearer $(HURL_user_token)"
export HURL_user_token
export HURL_host = http://localhost:3000

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
	cargo doc --package=query-server --open

# Format

fmt: fmt-server

fmt-server:
	cargo fmt --all --package=query-server -- --check

fmt-cli:
	cargo fmt --all --package=query -- --check

# Hurl

install-hurl:
	cargo install hurl

hurl: clean-hurl-dbs
	hurl --verbose --continue-on-error --file-root hurl $(ARGUMENTS)

hurl-test: clean-hurl-dbs
	hurl --test --continue-on-error --file-root hurl $(ARGUMENTS)

hurl-test-all: clean-hurl-dbs
	hurl --test --continue-on-error --file-root hurl hurl/**/*.hurl hurl/**/**/*.hurl

# Lint

lint:
	cargo clippy --all-targets --all-features --workspace

# Release

build-server:
	cargo build --package=query-server --release

build-cli:
	cargo build --package=query --profile dist

# Run

install-watch:
	cargo install cargo-watch

run:
	RUST_LOG=info cargo run --package=query-server -q | bunyan

run-cli:
	RUST_LOG=info cargo run --package=query

run-release:
	RUST_LOG=info cargo run --package=query-server --release -q | bunyan

run-cli-release:
	RUST_LOG=info cargo run --package=query --profile dist

dev-cli:
	cargo watch --ignore .dbs -x check -x clippy --shell "make run-cli -s"

dev:
	cargo watch -c --ignore .dbs -x check -x clippy --shell "RUST_LOG=debug cargo run --package=query-server | bunyan"

dev-proxy:
	export QUERY_SERVER_PROXY=true && cargo watch --ignore .dbs --shell "make run -s" & make dev-bun

dev-bun:
	touch .dbs/kv.sql
	export QUERY_SERVER_DBS_PATH=".dbs"
	bun examples/proxy/index.ts

# Test

install-nextest:
	cargo install cargo-nextest --locked

test:
	cargo test -- --test-threads=1

nextest:
	cargo nextest run

nextest-query:
	cargo nextest run --package=query

nextest-query-server:
	cargo nextest run --package=query-server

nextest-match:
	cargo nextest run --filter-expr 'test($(ARGUMENTS))'

test-watch:
	cargo watch -c -s "make test"

# Tag

tag:
	perl -pi -e 's/version = "$(call GET_ARGUMENT,1)"/version = "$(call GET_ARGUMENT,2)"/g' ./Cargo.toml
	perl -pi -e 's/version = "$(call GET_ARGUMENT,1)"/version = "$(call GET_ARGUMENT,2)"/g' ./crates/cli/src/main.rs
	cargo check --workspace
	git add Cargo.lock
	git add Cargo.toml
	git add crates/cli/src/main.rs
	git commit -m "release: version $(call GET_ARGUMENT,2)"
	git push --force-with-lease
	git tag v$(call GET_ARGUMENT,2)
	git push --tags

tag-delete:
	git tag -d v$(ARGUMENTS)
	git push origin --delete v$(ARGUMENTS)

# catch anything and do nothing
%:
	@: