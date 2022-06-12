NAME = what-to-code-cli
OUTPUT = what-to-code
INSTALL_DIR = /usr/bin

all: build install

build:
	cargo build --release

build-debug:
	cargo build

install:
	sudo cp target/release/$(NAME) $(INSTALL_DIR)/$(OUTPUT)

uninstall:
	sudo rm $(INSTALL_DIR)/$(OUTPUT)