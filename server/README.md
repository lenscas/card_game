## Card game
This is the server for my yet to be named roguelike card game. This is where the most magic happens. For example what encounters people get, deck storage and battlers are procesed here. The results are then shared with the client so it can render it to the user.


## file structure

`assets` This is where files go that the client can download.

`database` Here you can find every file needed to setup the database.

`lua` The lua parts of the server.

`src` the rust code of the server


## Programs/Libraries needed.

- [rustup](https://www.rust-lang.org/tools/install) to download latest stable rust compiler.

- Cargo, Will be installed automatically by rustup. It manages direct dependencies.

- [sqlx-cli](https://crates.io/crates/sqlx-cli), this helps create and update the database.

- [postgresql]](https://www.postgresql.org/download/), this is the database used by the server.

- [inkscape](https://inkscape.org/), this turns the svg files that cards are made out of into .png files that the client can actually use.

- [lua5.3](https://www.lua.org/start.html), This is used to prepare the cards/runes.

- [luarocks](https://luarocks.org/), is used to download the next lua dependencies. Can be skipped if you have other ways to accuire them.

- *[luasql-postgres](https://luarocks.org/modules/tomasguisasola/luasql-postgres), used to insert the cards to the database after they are processed. Making the changes accesible to the server.

- [lua-json](https://luarocks.org/modules/jiyinyiyong/json-lua), cards are turned into json. This dependency is used to do that.

- [luafilesystem](https://luarocks.org/modules/hisham/luafilesystem), otherwise lua has no way to get every file in a folder.

* Note that luasql-postgres may be a bit anoying to install through luarocks. If you use linux there is a good chance a version is available through your packamanager that you can use.

## Setup

1. install every dependency listed above.

2. make a copy of .env.example and replace the database connection string with your own. The rest can generally stay the same ***UNLESS*** you plan to make the server public. In that case you ***MUST*** edit the login token and pepper values as well.

3. run `sqlx database create` followed by `sqlx migrate run` to create and setup the database.

4. run `cd lua && lua compiler.lua` to process the cards and runes. This turns the lua files into json, prepares the images and updates the database to include the cards.

5. at this point you should be able to run `cargo run` to run the server.

Now, you should have a working server that is ready to accept users.

## How to update:

1. install/update any new/updated dependencies from the list above.

2. update your .env file so that everything listed in .env.example has a value.

3. run `sqlx migrate run` to update the database to the new schema

4. run `cd lua && lua compiler.lua` to update the cards.

## Tips:
There are 2 scripts that make use of `cargo-watch` to automatically recompile things when changes are made. cargo-watch can be installed with `cargo install cargo-watch`

the scripts are

`watch_server.sh` this script automatically runs `cargo run` whenever you have a change in the rust source code

`watch_cards.sh` this script automatically processes the cards whenever there is a change in the processor or the cards.
