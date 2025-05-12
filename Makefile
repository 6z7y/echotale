install:
	cargo build --release
	sudo install -m755 target/release/echotale /usr/bin/echotale

uninstall:
	sudo rm -f /usr/bin/echotale

clean:
	cargo clean
