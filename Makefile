# Makefile для сборки проекта NationOfLastLand-Core на Rust

.PHONY: all build release clean run test check

# По умолчанию сборка в режиме debug
all: build

# Сборка в режиме debug
build:
	cargo build

# Сборка в режиме release
release:
	cargo build --release

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
