# Makefile for dpc (Design Parity Checker)

PREFIX ?= $(HOME)/.cargo
BINDIR ?= $(PREFIX)/bin
CARGO ?= cargo

.PHONY: all build release check test clean install uninstall

all: build

build:
	$(CARGO) build

release:
	$(CARGO) build --release

check:
	$(CARGO) check
	$(CARGO) clippy --all-targets --all-features

test:
	$(CARGO) test

clean:
	$(CARGO) clean

install: release
	install -d $(DESTDIR)$(BINDIR)
	install -m 755 target/release/dpc $(DESTDIR)$(BINDIR)/dpc

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/dpc
