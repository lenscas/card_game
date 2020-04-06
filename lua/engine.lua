local ai = battle:get_ai()
local player = battle:get_player()
local aiCard = battle:get_ai_card()
local playerCard = battle:get_player_card(chosenCard)
local inOrder = {}

local aiSpeed = aiCard:get_speed()
local playerSpeed = playerCard:get_speed()

local SMALL_RUNE_BASE_FOLDER = "./compiled/small_runes/code/"

function load_small_rune_code(rune)
	local name = rune:get_name()
	return dofile(SMALL_RUNE_BASE_FOLDER .. name .. ".lua")
end

function addPercentage(current, percentage)
	return current + (current / 100 * percentage)
end

function subPercentage(current, percentage)
	return current - (current / 100 * percentage)
end

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

function process_speed_runes(caster, oponent, card)
	local casterRunes = caster:get_runes()
	local extraSpeed = 0
	for k, v in pairs(casterRunes) do
		local code = load_small_rune_code(v)
		if code.owner_modify_speed ~= nil then
			extraSpeed = extraSpeed + code:owner_modify_speed(v, playerCard, player)
		end
	end
	local oponentRunes = oponent:get_runes()
	for k, v in pairs(oponentRunes) do
		local code = load_small_rune_code(v)
		if code.oponent_modify_speed ~= nil then
			extraSpeed = extraSpeed - code:owner_modify_speed(v, playerCard, oponent)
		end
	end
	return extraSpeed
end

print("before player", playerSpeed)
playerSpeed = addPercentage(playerSpeed, process_speed_runes(player, ai, playerCard))
print("after player", playerSpeed)
aiSpeed = addPercentage(aiSpeed, process_speed_runes(ai, player, aiCard))
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
