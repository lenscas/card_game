local Rust = require("types/Rust")

global battle : Rust.Field;
global chosenCard : Rust.Card;

require("engine/battle"):init(battle,chosenCard)
local turn = require"engine/turn"

return turn
