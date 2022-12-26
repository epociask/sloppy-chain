
build-bin:
	@ rustc ./node/src/main.rs

build-cargo:
	@ cargo build --verbose
