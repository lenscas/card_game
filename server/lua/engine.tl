package.path = './lua/?.lua;' .. package.path
package.path = "./lua?.tl" .. package.path

local Rust = require("types/rust")

global battle : Rust.Field;
global chosenCard : Rust.Card;

require("engine/battle"):init(battle,chosenCard)
local turn = require"engine/turn"

return turn()
