.PHONY: all frontend-install frontend-dev frontend-build backend-dev backend-release run run-release test clean

# Default target
all: frontend-build backend-release

# Frontend targets
frontend-install:
	cd frontend && npm install

frontend-dev:
	cd frontend && npm run dev

frontend-build:
	cd frontend && npm run build

# Backend targets
backend-dev:
	cargo build

backend-release:
	cargo build --release

# Run targets
run: frontend-build backend-dev
	cargo run

run-release: frontend-build backend-release
	./target/release/mermaid_render

# Testing
test:
	cd frontend && npm test
	cargo test

# Cleanup
clean:
	cd frontend && rm -rf node_modules build
	cargo clean

# Install all dependencies
install: frontend-install
	cargo fetch

# Help target to show available commands
help:
	@echo "Available commands:"
	@echo "  make install         - Install all dependencies"
	@echo "  make frontend-install - Install frontend dependencies"
	@echo "  make frontend-dev    - Start frontend development server"
	@echo "  make frontend-build  - Build frontend for production"
	@echo "  make backend-dev     - Build backend in debug mode"
	@echo "  make backend-release - Build backend in release mode"
	@echo "  make run             - Run in development mode"
	@echo "  make run-release     - Run in release mode"
	@echo "  make test            - Run tests"
	@echo "  make clean           - Clean build artifacts"
	@echo "  make help            - Show this help message"
