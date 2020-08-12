.DEFAULT_GOAL := build-release
.PHONY: all run build build-gtk build-iced build-release build-iced-release build-gtk-release clean install uninstall

build: build-iced

build-gtk:
	cargo build --features gtk_f
build-iced:
	cargo build --features iced_f

build-release: build-iced-release

build-iced-release:
	cargo build --features iced_f --release
build-gtk-release:
	cargo build --features gtk_f --release

clean:
	cargo clean

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