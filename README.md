mutt-ottolangy
==============

Set Mutt’s attribution format string based on the language of the message you’re
replying to.

If the input message is in English, the attribution will look like:

	On Mar 19, 2021, at 01:55 PM +0000, Philip J. Fry <philip.j.fry@example.com> wrote:

In French, it appears as:

	Le 19 mar. 2021 à 13:55 +0000, Philippe J. Fry <philippe.j.fry@example.com> a écrit:


## Install
On Mac OS X, Ottolangy can be installed with Homebrew:

	$ brew install teddywing/formulae/ottolangy

To compile from source or install on other platforms:

	$ cargo install --git https://github.com/teddywing/ottolangy.git

Copy the contents of the included [muttrc] file or source it. The included
macros override Mutt’s reply commands.


## Uninstall

	$ cargo uninstall ottolangy


## Caveats
Attribution format strings are currently hard-coded. Only English and French are
supported. If the language isn’t recognised as French, attribution defaults to
English.


## License
Copyright © 2021 Teddy Wing. Licensed under the GNU GPLv3+ (see the included
COPYING file).


[muttrc]: ./muttrc
