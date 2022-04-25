update:
	git checkout master
	git pull
	make build
	@echo "Updated and rebuild succesfully"
build:
	cargo b -r
install:
	cp ./target/release/accuracy-calculator /usr/bin
	@echo "Installed"
uninstall:
	rm /usr/bin/accuracy-calculator
	@echo "Uninstalled"
