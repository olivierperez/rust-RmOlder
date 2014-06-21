.PHONY=all

all: clean tests doc build run
clean:
	rm -Rf target
	mkdir -p target/bin target/test

tests:
	rustc rmolder.rs --test --out-dir target/test
	./target/test/rmolder

doc:
	rustdoc rmolder.rs

build:
	rustc --out-dir target/bin rmolder.rs
	rustc --out-dir target/bin main.rs -L target/bin

run:
	./target/bin/main -d test -a 1000000 --dry

