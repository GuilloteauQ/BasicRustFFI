all: run

run: src/lib.rs main.c
	cargo build --lib --release
	gcc main.c -L target/release -l add -o run

clean:
	cargo clean & rm run
