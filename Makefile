VERSION=1.0
NAME=rust-makefile
EXEC=rust-exec
PREFIX=$(HOME)/.local

default: build_release

clean:
	@echo "Cleaning build dir"
	@rm -rf target/*
	@echo "Cleaning using cargo"
	@cargo clean
check:
	@echo "Checking $(NAME)"
	@cargo check
build_release:
	@echo "Building release: $(VERSION)"
	@./build.sh
run:
	@echo "Running debug"
	@docker compose up note-app -d
runk6:
	@docker compose -f docker-compose-k6.yml up
downk6:
	@docker compose -f docker-compose-k6.yml down
debug_run: build_debug run
