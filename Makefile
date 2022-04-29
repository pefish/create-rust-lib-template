

DEFAULT: build-release

build-release:
	cargo +nightly build --release

build:
	cargo +nightly build

publish:
	cargo +nightly publish

test:
	cargo +nightly test