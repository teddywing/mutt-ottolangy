# Copyright (c) 2021  Teddy Wing
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.


VERSION := $(shell egrep '^version = ' Cargo.toml | awk -F '"' '{ print $$2 }')
TOOLCHAIN := $(shell fgrep default_host_triple $(HOME)/.rustup/settings.toml | awk -F '"' '{ print $$2 }')
SOURCES := $(shell find src -name '*.rs')

DEBUG_PRODUCT := target/debug/ottolangy
RELEASE_PRODUCT := target/release/ottolangy

MAN_PAGE := doc/ottolangy.1

DIST := $(abspath dist)
DIST_PRODUCT := $(DIST)/bin/ottolangy
DIST_MUTTRC := $(DIST)/etc/ottolangy/muttrc
DIST_MAN_PAGE := $(DIST)/share/man/man1/ottolangy.1


$(DEBUG_PRODUCT): $(SOURCES)
	cargo build

$(RELEASE_PRODUCT): $(SOURCES)
	cargo build --release


.PHONY: test
test: $(DEBUG_PRODUCT)
	prove -v -I./t


.PHONY: doc
doc: $(MAN_PAGE)

$(MAN_PAGE): doc/ottolangy.1.txt
	sed "s/\$$PREFIX/$$PREFIX/g" $< > "$<.tmp"
	a2x --no-xmllint --format manpage "$<.tmp"
	rm "$<.tmp"


.PHONY: dist
dist: $(DIST_PRODUCT) $(DIST_MUTTRC) $(DIST_MAN_PAGE)

$(DIST):
	mkdir -p $@

$(DIST)/bin: | $(DIST)
	mkdir -p $@

$(DIST)/etc/ottolangy: | $(DIST)
	mkdir -p $@

$(DIST)/share/man/man1: | $(DIST)
	mkdir -p $@

$(DIST_PRODUCT): $(RELEASE_PRODUCT) | $(DIST)/bin
	cp $< $@

$(DIST_MUTTRC): muttrc | $(DIST)/etc/ottolangy
	cp $< $@

$(DIST_MAN_PAGE): $(MAN_PAGE) | $(DIST)/share/man/man1
	cp $< $@


.PHONY: pkg
pkg: ottolangy_$(VERSION)_$(TOOLCHAIN).tar.bz2

ottolangy_$(VERSION)_$(TOOLCHAIN).tar.bz2: dist
	tar cjv -s /dist/ottolangy_$(VERSION)_$(TOOLCHAIN)/ -f $@ dist
