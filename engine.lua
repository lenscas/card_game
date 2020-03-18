local ai = battle:get_ai()
local player = battle:get_player()
local aiCard = battle:get_ai_card()
local playerCard = battle:get_player_card(chosenCard)
local inOrder = {}

local aiSpeed = aiCard:get_speed()
local playerSpeed = playerCard:get_speed()

function giveRuneToPlayer(player, runeName)
	local handler = io.open("./compiled_small_runes/config/" .. runeName .. ".lua")
	local str = handler.read("*a")
	handler:close()
	local asJson = json.decode(str)
	player:add_rune(asJson)
end

--[[
function loadSmallRune(runeName)
	local handler = io.open("./compiled_small_runes/code/" .. runeName .. ".lua")
	local all = handler.read("*a")
	handler:close()
	local asFunc, err = load("return " .. all)
	if err then
		error(err .. "\n type : small rune \n name: " .. runeName)
	end


end
--]]
local function createRunCardFunc(card, battle, owner, oponent)
	return function()
		print(card:get_name())
		local code = card:get_code()
		print(code)
		local asFunc = load("return " .. code)()
		print(asFunc)
		asFunc.func(card, battle, owner, oponent)
		battle:save_ai(ai)
		battle:save_player(player)
	end
end

--TODO better selection on who goes first

if aiSpeed == playerSpeed then
	if math.random(0, 1) == 0 then
		aiSpeed = 0
		playerSpeed = 1
	else
		aiSpeed = 1
		playerSpeed = 0
	end
end

local aiCardAsFunc = createRunCardFunc(aiCard, battle, ai, player)
local playerCardAsFunc = createRunCardFunc(playerCard, battle, player, ai)

if aiSpeed > playerSpeed then
	table.insert(inOrder, aiCardAsFunc)
	table.insert(inOrder, playerCardAsFunc)
else
	table.insert(inOrder, playerCardAsFunc)
	table.insert(inOrder, aiCardAsFunc)
end
for key, card in ipairs(inOrder) do
	card()
	if battle:has_ended() then
		break
	end
end
print("quick update")
return battle
