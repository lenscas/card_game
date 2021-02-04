package.path = './lua/?.lua;' .. package.path
global battle : Rust.Field;
global chosenCard : Rust.Card;

require("engine/battle"):init(battle,chosenCard)
local turn = require"engine/turn"

return {turn()}
