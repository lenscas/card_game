## Card game
This is the server for my yet to be named roguelike card game. Not much to see here yet though :(

## file structure

`assets` This is where files go that the client can download.

`database` Here you can find every file needed to setup the database.

`lua` The lua parts of the server.

`src` the rust code of the server


## Programs/Libraries needed.

use `rustup` to download the latest stable rust compiler and `cargo`. Cargo will manage the rust dependencies for you.

Further you need `lua5.3` as well as `lua-sql`, `lua-filesystem` and `json-lua` these can be installed using `luarocks`

You also need `postgresql` and `inkscape`

## Setup

Make a `.env` file based on `.env.example` with the placeholders replaced for actual values.

use `database/schema.sql` to setup the database structure. Note this does not contain any (test) data.

Then, run `cd lua && lua compiler.lua` to compile the cards.

Finally, run `cargo run` to start the server.

## Tips:
`get_db_structure.sh` is a script that can make a dump of the database schema and places this is `./database/schema.sql`. Don't forget to run this before comitting if you made changes to the database.

There are also 2 scripts that make use of `cargo-watch` to automatically recompile things when changes are made. You can install this using `cargo install cargo-watch`

These scripts are

`watch_server.sh` this script automatically runs `cargo run` whenever you have a change in the rust source code

`watch_cards.sh` this script automatically processes the cards whenever there is a change in the processor or the cards.