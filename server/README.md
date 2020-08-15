## Card game
This is the server for my yet to be named roguelike card game. This is where the most magic happens. For example what encounters people get, deck storage and battlers are procesed here. The results are then shared with the [client](https://github.com/lenscas/card_game_client) so it can render it to the user.


## file structure

`assets` This is where files go that the client can download.

`database` Here you can find every file needed to setup the database.

`lua` The lua parts of the server.

`src` the rust code of the server


## Programs/Libraries needed.

use [rustup](https://www.rust-lang.org/tools/install) to download the latest stable rust compiler and `cargo`. Cargo will manage the rust dependencies for you.

Further you need [lua5.3](https://www.lua.org/start.html) as well. You also need to download a few lua specific libraries. These can be installed using [luarocks](https://luarocks.org/). These are * [luasql-postgres](https://luarocks.org/modules/tomasguisasola/luasql-postgres) , [lua-json](https://luarocks.org/modules/jiyinyiyong/json-lua) and [luafilesystem](https://luarocks.org/modules/hisham/luafilesystem)

You also need [postgresql](https://www.postgresql.org/download/) and [inkscape](https://inkscape.org/)

* Note that luasql-postgres may be a bit anoying to install through luarocks. If you use linux there is a good chance a version is available through your packamanager that you can use.

## Setup

make a copy of `.env.example` called `.env` and replace the conection string for the database with your own. The other 2 can stay the same for development, however if you want to host a public facing version you **must** change them.

use `database/schema.sql` to setup the database structure (`psql dbname < ./database/schema.sql`). Note this does not contain any (test) data.

Next, run `cd lua && lua compiler.lua` to process the cards and runes. This makes new files based on the human readable versions, sets the database up to contain them and prepares images for them.

Finally, run `cargo run` to start the server.

Now, you should have a working server that is ready to accept users.

## Tips:
`get_db_structure.sh` is a script that can make a dump of the database schema and places this is `./database/schema.sql`. Don't forget to run this before comitting if you made changes to the database.

There are also 2 scripts that make use of `cargo-watch` to automatically recompile things when changes are made. You can install this using `cargo install cargo-watch`

These scripts are

`watch_server.sh` this script automatically runs `cargo run` whenever you have a change in the rust source code

`watch_cards.sh` this script automatically processes the cards whenever there is a change in the processor or the cards.
