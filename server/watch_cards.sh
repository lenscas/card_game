#!/bin/sh

cargo watch -c -w ./src -w ./lua/cards -w ./lua/compiler.lua -w ./lua/compiler -s "cd lua && lua compiler.lua"