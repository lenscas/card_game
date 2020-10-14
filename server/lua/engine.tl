package.path = './lua/?.lua;' .. package.path
package.path = "./lua?.tl" .. package.path

global battle : any;
global chosenCard : any;

require("engine/battle"):init(battle,chosenCard)
local turn = require"engine/turn"

return turn()
