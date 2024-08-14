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

# Add this at the beginning of your Makefile
.PHONY: help
help:
	@echo "Available commands:"
	@echo
	@echo "Build:"
	@echo "  build-server            - Build the query server in release mode"
	@echo "  build-server-watch      - Watch and build the query server, with debug logging"
	@echo "  build-cli               - Build the CLI with the dist profile"
	@echo
	@echo "Changelog:"
	@echo "  update-changelog        - Update CHANGELOG.md with changes between the last two release commits"
	@echo
	@echo "Clean:"
	@echo "  clean                   - Remove Cargo.lock and clean the build artifacts"
	@echo "  clean-dbs               - Remove all database files"
	@echo "  clean-hurl-dbs          - Remove Hurl-specific database files"
	@echo
	@echo "CLI:"
	@echo "  install-esbuild         - Install esbuild"
	@echo "  cli                     - Run the CLI"
	@echo
	@echo "Coverage:"
	@echo "  install-llvm-cov        - Install cargo-llvm-cov"
	@echo "  coverage-clean          - Clean coverage data"
	@echo "  coverage                - Run coverage analysis"
	@echo "  coverage-watch          - Watch and run coverage analysis"
	@echo "  coverage-report         - Generate coverage report"
	@echo "  coverage-html           - Generate HTML coverage report"
	@echo "  coverage-lcov           - Generate LCOV coverage report"
	@echo
	@echo "Distribution:"
	@echo "  install-cargo-dist      - Install cargo-dist"
	@echo "  dist-plan               - Plan distribution"
	@echo
	@echo "Documentation:"
	@echo "  docs                    - Generate and open documentation"
	@echo
	@echo "Formatting:"
	@echo "  fmt                     - Format all code"
	@echo "  fmt-server              - Format server code"
	@echo "  fmt-cli                 - Format CLI code"
	@echo
	@echo "Hurl:"
	@echo "  install-hurl            - Install Hurl"
	@echo "  hurl                    - Run Hurl tests"
	@echo "  hurl-test               - Run Hurl tests in test mode"
	@echo "  hurl-test-all           - Run all Hurl tests in test mode"
	@echo
	@echo "Linting:"
	@echo "  lint                    - Run clippy on all targets"
	@echo
	@echo "npm:"
	@echo "  npm-publish             - Publish npm packages"
	@echo "  npm-prerelease          - Publish prerelease npm packages"
	@echo "  npm-un-prerelease       - Unpublish prerelease npm packages"
	@echo
	@echo "Run:"
	@echo "  install-watch           - Install cargo-watch"
	@echo "  run                     - Run the server"
	@echo "  run-cli                 - Run the CLI"
	@echo "  run-release             - Run the server in release mode"
	@echo "  run-cli-release         - Run the CLI in release mode"
	@echo "  dev                     - Run the server in development mode"
	@echo "  dev-build               - Watch and build the server"
	@echo "  dev-cli                 - Watch and run the CLI"
	@echo "  dev-proxy               - Run the server with proxy enabled"
	@echo "  dev-bun                 - Run the Bun development server"
	@echo
	@echo "Testing:"
	@echo "  install-nextest         - Install cargo-nextest"
	@echo "  test                    - Run tests"
	@echo "  nextest                 - Run tests with nextest"
	@echo "  nextest-query           - Run query tests with nextest"
	@echo "  nextest-query-server    - Run query-server tests with nextest"
	@echo "  nextest-match           - Run specific tests with nextest"
	@echo "  test-watch              - Watch and run tests"
	@echo
	@echo "Tagging:"
	@echo "  tag                     - Create a new version tag"
	@echo "  tag-delete              - Delete a version tag"
	@echo "  tag-rollback            - Rollback a version tag"
	@echo
	@echo "For more details on each command, check the Makefile"

# Build

build-server:
	cargo build --package=query-server --release

build-server-watch:
	cargo watch -c --ignore .dbs -x check -x clippy --shell "RUST_LOG=debug cargo build --package=query-server | bunyan"

build-cli:
	cargo build --package=query --profile dist

# Changelog

update-changelog:
	@commits=$$(git log --grep="release: version" --format="%H" -n 2); \
	if [ $$(echo "$$commits" | wc -l) -lt 2 ]; then \
		echo "Error: Less than two 'release' commits found."; \
		exit 1; \
	fi; \
	last=$$(echo "$$commits" | head -n 1); \
	prev=$$(echo "$$commits" | tail -n 1); \
	git cliff $$prev..$$last --prepend CHANGELOG.md; \
	echo "CHANGELOG.md has been updated with changes between commits:"; \
	echo "Previous: $$prev"; \
	echo "Latest: $$last"

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

hurl-function:
	@name="$(ARGUMENTS)"; \
	if [ -f "hurl/function/$$name.js" ]; then \
		output=$$(node hurl/function/$$name.js); \
		sed -i "s/\"function\": \[.*\]/\"function\": $$output/" hurl/function/$$name.hurl 2>/dev/null && \
		echo "Updated hurl/function/$$name.hurl" || \
		echo "Error: hurl/function/$$name.hurl not found"; \
	else \
		echo "Error: hurl/function/$$name.js not found"; \
	fi

hurl-bytes:
	node hurl/file_to_bytes.mjs $(call GET_ARGUMENT,1) $(call GET_ARGUMENT,2)

# Lint

lint:
	cargo clippy --all-targets --all-features --workspace

# npm

npm-publish:
	npm publish https://github.com/gc-victor/query/releases/download/v$(ARGUMENTS)/query-npm-package.tar.gz
	npm publish https://github.com/gc-victor/query/releases/download/v$(ARGUMENTS)/query-server-npm-package.tar.gz

npm-prerelease:
	@if [ "$(findstring prerelease,$(ARGUMENTS))" = "prerelease" ]; then \
		npm publish https://github.com/gc-victor/query/releases/download/v$(ARGUMENTS)/query-npm-package.tar.gz --tag prerelease-$(VERSION) ;\
		npm publish https://github.com/gc-victor/query/releases/download/v$(ARGUMENTS)/query-server-npm-package.tar.gz --tag prerelease-$(VERSION) ;\
	fi

npm-un-prerelease:
	@if [ "$(findstring prerelease,$(ARGUMENTS))" = "prerelease" ]; then \
		npm unpublish @qery/query@$(ARGUMENTS) --force ;\
		npm unpublish @qery/query-server@$(ARGUMENTS) --force ;\
	fi

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

dev:
	cargo watch -c --ignore .dbs -x check -x clippy --shell "RUST_LOG=debug cargo run --package=query-server | bunyan"

dev-build:
	cargo watch -c --ignore .dbs -x check -x clippy --shell "cargo build --package=query-server"

dev-cli:
	cargo watch --ignore .dbs -x check -x clippy --shell "make run-cli -s"

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
	@if [ "$(findstring prerelease,$(call GET_ARGUMENT,2))" = "prerelease" ]; then \
		perl -pi -e 's/targets = \["aarch64\-apple\-darwin", "x86_64\-apple\-darwin", "x86_64\-unknown\-linux\-gnu", "x86_64\-pc\-windows\-msvc"\]/targets = \["x86_64\-unknown\-linux\-gnu"\]/g' ./Cargo.toml; \
    fi
	cargo check --workspace
	git add Cargo.lock
	git add Cargo.toml
	git commit -m "release: version $(call GET_ARGUMENT,2)"
	git push --force-with-lease
	git tag v$(call GET_ARGUMENT,2)
	git push --tags

tag-delete:
	@read -p "Are you sure you want to delete the tag version $(ARGUMENTS)? [Y/n] " REPLY; \
	if [ "$$REPLY" = "Y" ] || [ "$$REPLY" = "y" ] || [ "$$REPLY" = "" ]; then \
		git tag -d v$(ARGUMENTS); \
		git push origin --delete v$(ARGUMENTS); \
	else \
		echo "Aborted."; \
	fi

tag-rollback:
	@read -p "Are you sure you want to rollback the tag version $(ARGUMENTS)? [Y/n] " REPLY; \
    if [ "$$REPLY" = "Y" ] || [ "$$REPLY" = "y" ] || [ "$$REPLY" = "" ]; then \
        git reset --soft HEAD~1; \
		git reset HEAD Cargo.lock; \
		git reset HEAD Cargo.toml; \
		git checkout -- Cargo.lock; \
		git checkout -- Cargo.toml; \
		git tag -d v$(ARGUMENTS); \
		git push origin --delete v$(ARGUMENTS); \
		git push --force-with-lease; \
    else \
        echo "Aborted."; \
    fi

# catch anything and do nothing
%:
	@: