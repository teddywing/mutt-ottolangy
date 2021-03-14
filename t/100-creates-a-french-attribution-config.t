#!/usr/bin/env perl -w

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


use strict;

use Test::More;

use Bin qw($BIN);

my $attribution_muttrc_path = "$ENV{'HOME'}/.local/share/ottolangy/attribution.muttrc";
my $attribution_config = q(set attribution = "Le %{%e %b. %Y à %H:%M %Z}, %f a écrit:"
);

# Remove any existing Ottolangy muttrc file.
unlink $attribution_muttrc_path;

system("$BIN < ./t/data/french.eml");
ok !$?;

my $generated_config;
{
	local $\;
	open my $fh, '<', $attribution_muttrc_path or die $!;
	$generated_config = <$fh>;
}
is $generated_config, $attribution_config;


done_testing;
