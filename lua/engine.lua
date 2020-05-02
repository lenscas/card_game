package.path = './lua/?.lua;' .. package.path

require("engine/battle"):init(battle,chosenCard)
local turn = require"engine/turn"

return turn()
