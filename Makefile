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


SOURCES := $(shell find src -name '*.rs')

DEBUG_PRODUCT := target/debug/ottolangy

MAN_PAGE := doc/ottolangy.1


$(DEBUG_PRODUCT): $(SOURCES)
	cargo build

.PHONY: test
test: $(DEBUG_PRODUCT)
	prove -v -I./t


.PHONY: doc
doc: $(MAN_PAGE)

$(MAN_PAGE): doc/ottolangy.1.txt
	sed "s/\$$PREFIX/$$PREFIX/g" $< > "$<.tmp"
	a2x --no-xmllint --format manpage "$<.tmp"
	rm "$<.tmp"
