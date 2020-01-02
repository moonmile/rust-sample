
sample: src/main.c libhello.a
	gcc -o sample src/main.c libhello.a -lpthread -ldl 
libhello.a: src/lib.rs
	cargo build 
	cp target/debug/libhello.a ./libhello.a
