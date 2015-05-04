all:
	cargo build
	target/debug/gen_bindings > rust.cpp
	g++ -c rust.cpp cpp.cpp -fPIC
	rm -f libcpp.a
	ar rvs libcpp.a rust.o cpp.o
	rustc main.rs -L "."
	echo
	./main
