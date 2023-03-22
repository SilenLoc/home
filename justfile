#!/usr/bin/env just --justfile
set dotenv-load
export PATH := "./node_modules/.bin:" + env_var('PATH')

@_list:
	just --list --unsorted

# Downcload dependencies
install:
    npm ci --locked

# Serve the sources and watch changes
run: install
    npm run dev -- --open

# Run the tests
test: install
	npm test

# Run linters
format: install
    npm run format

# Run linters
lint: install
    npm run lint

# Build release artifacts
build: install
    npm run build

# Install development tools
install-dev-tools:
    npx playwright install