SHELL := /bin/bash
BUILD = target
WASM_PATH = target/wasm32-unknown-unknown/release
OUT = dist
BIN = bin
TITLE = WASM Lander

.PHONY: help install-tools build clean
.DEFAULT_GOAL = build

help: ## ๐ฌ This help message :)
	@figlet $@ || true
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

lint: ## ๐ Check for linting and formatting errors
	@figlet $@ || true
	@cargo fmt --all -- --check
	@cargo clippy

lint-fix: ## ๐ง Fix linting and formatting errors
	@figlet $@ || true
	@cargo fmt --all --
	@cargo clippy --fix --allow-dirty

install-tools: ## ๐ฎ Install dev tools and pre-reqs
	@figlet $@ || true
	@wget -q https://github.com/aduros/wasm4/releases/latest/download/w4-linux.zip
	@unzip -o ./w4-linux.zip -d ./bin/
	@rm ./w4-linux.zip
	@cd $(BIN); wget -q https://github.com/WebAssembly/binaryen/releases/download/version_91/binaryen-version_91-x86_64-linux.tar.gz -O - | tar -xz
	@mv $(BIN)/binaryen-version_91/wasm-opt $(BIN)/wasm-opt
	@rm -rf $(BIN)/binaryen*
	@which cargo > /dev/null || { echo "ERROR! Rust is not installed!"; exit 1; }
	@which rustup > /dev/null || { echo "ERROR! Rust is not installed!"; exit 1;}

build: ## ๐จ Build the game cart WASM
	@figlet $@ || true
	cargo build --release

clean: ## ๐งน Clean up build artifacts
	@figlet $@ || true
	@rm -rf $(BUILD)
	@rm -rf $(OUT)

run: build ## ๐ Run the game and start the web server
	@figlet $@ || true
	@$(BIN)/w4 run $(WASM_PATH)/cart.wasm --no-qr

watch: ## ๐ Run the game with reload on file change
	@figlet $@ || true
	@$(BIN)/w4 watch --no-qr

publish: build ## ๐ Bundle for distribution (exe and HTML)
	@figlet $@ || true
	@mkdir -p dist
	@rm -rf $(OUT)/cart-opt.wasm
	@$(BIN)/wasm-opt $(WASM_PATH)/cart.wasm -o $(OUT)/cart-opt.wasm -Oz --strip-dwarf --strip-producers
	@echo ๐พ Optimised file is: $(shell stat --printf="%s" $(OUT)/cart-opt.wasm) bytes
	@$(BIN)/w4 bundle $(OUT)/cart-opt.wasm --html $(OUT)/index.html --title "$(TITLE)" --icon-file assets/icon.png
	@$(BIN)/w4 bundle $(OUT)/cart-opt.wasm --linux $(OUT)/game --title "$(TITLE)"
	@$(BIN)/w4 bundle $(OUT)/cart-opt.wasm --windows $(OUT)/game.exe --title "$(TITLE)"