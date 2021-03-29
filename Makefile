napi:
	cargo build --release
	mv target/release/libnapi_test.so libnapi_test.so.node
