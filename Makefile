.DEFAULT_GOAL := build-release
.PHONY: all run build build-release clean install uninstall

build:
	cargo build

build-release:
	cargo build --release

clean:
	rm -rf target/

install:
	cp target/release/yavanna /usr/bin/yavanna
	mkdir -p /opt/yavanna/misc
	cp misc/icon.png /opt/yavanna/misc/icon.png
	cp misc/yavanna.desktop /usr/share/applications/yavanna.desktop

uninstall:
	rm /usr/bin/yavanna
	rm -r /opt/yavanna
	rm /usr/share/applications/yavanna.desktop

run:
	cargo run