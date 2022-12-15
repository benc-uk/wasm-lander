# Rust WASM Lander

A game written in Rust for the [WASM-4](https://wasm4.org) fantasy console.

![](./assets/screenshot.png)


The game is published as HTML and hosted on GitHub Pages  
### [🌚 Play live version!](http://code.benco.io/wasm-lander/)


## Getting Started

- Install [Rust & Cargo](https://www.rust-lang.org/tools/install)
- run `make install-tools` to install **w4** and **wasm-opt** locally

## Building

Build the cart by running:

```shell
make build
```

Then run the browser HTML version with:

```shell
make run
```

This will auto start a web server and open the game in your browser

## Bundling

To output binaries for Linux, Windows and an standalone HTML page, run:

```shell
make publish
```

## Make

```text
❯ make help
 _          _       
| |__   ___| |_ __  
| '_ \ / _ \ | '_ \ 
| | | |  __/ | |_) |
|_| |_|\___|_| .__/ 
             |_|    
help                 💬 This help message :)
lint                 🔎 Check for linting and formatting errors
lint-fix             🧙 Fix linting and formatting errors
install-tools        🔮 Install dev tools and pre-reqs
build                🔨 Build the game cart WASM
clean                🧹 Clean up build artifacts
run                  🚀 Run the game and start the web server
watch                👀 Run the game with reload on file change
publish              🎁 Bundle for distribution (exe and HTML)
```

For more info about setting up WASM-4, see the [quickstart guide](https://wasm4.org/docs/getting-started/setup?code-lang=rust#quickstart).

## Links

- [Documentation](https://wasm4.org/docs): Learn more about WASM-4.
- [Snake Tutorial](https://wasm4.org/docs/tutorials/snake/goal): Learn how to build a complete game
  with a step-by-step tutorial.
- [GitHub](https://github.com/aduros/wasm4): Submit an issue or PR. Contributions are welcome!
