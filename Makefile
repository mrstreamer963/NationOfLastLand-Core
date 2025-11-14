# Makefile для сборки проекта NationOfLastLand-Core на Rust

.PHONY: all build release clean run test check wasm wasm-release

# По умолчанию сборка в режиме debug
all: build

# Сборка в режиме debug
build:
	cargo build

# Сборка в режиме release
release:
	cargo build --release

# Сборка для WebAssembly (debug)
wasm:
	cargo build --target wasm32-unknown-unknown

# Сборка для WebAssembly (release)
wasm-release:
	cargo build --target wasm32-unknown-unknown --release

# Очистка
clean:
	cargo clean

# Запуск
run:
	cargo run

# Тестирование
test:
	cargo test

# Проверка кода без сборки
check:
	cargo check
