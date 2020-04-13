local battle = battle or {}
local chosenCard = chosenCard or {}

local ai = battle:get_ai()
local player = battle:get_player()
local aiCard, aiCardIndex = battle:get_ai_card()
local playerCard, playerCardIndex = battle:get_player_card(chosenCard)
local inOrder = {}

local aiSpeed = aiCard:get_speed()
local playerSpeed = playerCard:get_speed()

local SMALL_RUNE_BASE_FOLDER = "./lua/compiled/small_runes/code/"

local function load_small_rune_code(rune)
	local name = rune:get_name()
	return dofile(SMALL_RUNE_BASE_FOLDER .. name .. ".lua")
end

function addPercentage(current, percentage)
	return current + (current / 100 * percentage)
end

function subPercentage(current, percentage)
	return current - (current / 100 * percentage)
end

local function run_rune_code(owner, index, rune, rune_func, params)
	index = index - 1
	print(rune)
	local code = load_small_rune_code(rune)
	if code[rune_func] then
		print("has func")
		local ret_value = {code[rune_func](code,table.unpack(params))}
		owner:save_rune(rune,index)
		return table.unpack(ret_value)
	end
end

function deal_damage(amount, from, to)
	local fromRunes = from:get_runes()
	local negatives = {}
	for k, v in pairs(fromRunes) do
		local buff =
			run_rune_code(
				from,
				k,
				v,
				"owner_modify_damage",
				{v, amount, from, to}
			) or 0
		if buff > 0 then
			amount = amount + (amount / 100 * buff)
		elseif buff < 0 then
			negatives[#negatives] = buff
		end
	end
	for _, v in pairs(negatives) do
		amount = amount + (amount / 100 * v)
	end
	local positives = {}
	local toRunes = to:get_runes()
	for k, v in pairs(toRunes) do
		local reduction = run_rune_code(
				from,
				k,
				v,
				"receiver_modify_damage",
				{v, amount, from, to}
			) or 0
		if reduction > 0 then
			amount = amount - (amount / 100 * reduction)
		elseif reduction < 0 then
			positives[#positives] = reduction
		end
	end
	for _, v in pairs(positives) do
		amount = amount - (amount / 100 * v)
	end
	to:deal_damage(amount)
end

local function createRunCardFunc(card, battle, owner, oponent)
	return function()
		print(card:get_name())
		local code = card:get_code()
		print(code)
		local asFunc = load("return " .. code)()
		print(asFunc)
		asFunc.func(card, battle, owner, oponent)
		owner:sub_mana(card:get_cost())
		battle:save_ai(ai)
		battle:save_player(player)
		return function()
			local runes = owner:get_runes()
			for k, v in pairs(runes) do
				run_rune_code(owner, k, v, "end_of_turn_effect", {v,battle, owner, oponent})
			end
		end
	end
end

local function process_speed_runes(caster, oponent, card)
	local casterRunes = caster:get_runes()
	local extraSpeed = 0
	for k, v in pairs(casterRunes) do
		extraSpeed = extraSpeed + (run_rune_code(
				caster,
				k,
				v,
				"owner_modify_speed",
				{v, card, caster}
			) or 0)
	end
	local oponentRunes = oponent:get_runes()
	for k, v in pairs(oponentRunes) do
		extraSpeed = extraSpeed - (run_rune_code(
				caster,
				k,
				v,
				"oponent_modify_speed",
				{v, card, caster}
			) or 0)
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
local runes = {}
for key, card in ipairs(inOrder) do
	table.insert(runes, card())
	if battle:has_ended() then
		break
	end
end

for key, rune in ipairs(runes) do
	rune()
	if battle:has_ended() then
		break
	end
end

player:remove_card(playerCardIndex)
ai:remove_card(aiCardIndex)
player:clean_up_runes()
ai:clean_up_runes()
player:add_mana(1)
ai:add_mana(1)
battle:save_ai(ai)
battle:save_player(player)
print("quick update")
return battle
