update:
	git checkout master
	git pull
	make build
	@echo "Updated and rebuild succesfully"
build:
	cargo b -r
install:
	cp ./target/release/accuracy-calculator ~/.local/bin
	@echo "Installed"
uninstall:
	rm ~/.local/bin/accuracy-calculator
	@echo "Uninstalled"
test:
	cargo test
	@echo "Tested"
