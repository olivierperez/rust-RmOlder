.PHONY=all

all: clean test doc build run
clean:
	mkdir -p target/bin target/test

test:
	rustc rmolder.rs --test --out-dir out_test
	./out_test/rmolder

doc:
	rustdoc rmolder.rs

build:
	rustc --crate-type=lib --out-dir out rmolder.rs
	rustc --out-dir out main.rs -L out

run:
	./out/main -d test -a 1000000 --dry

