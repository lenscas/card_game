local battle = require"engine/battle"
local maths = require"engine/math"
local runes = require"engine/runes"

local function createRunCardFunc(card, battle1, owner, oponent)
	print(card,battle1,owner,oponent)
	return function()
		print("cardname",card:get_name())
		local code = card:get_code()
		print("code",code)
		local asFunc = load("return " .. code)()
		print("as func", asFunc)
		asFunc.func(card, battle1, owner, oponent)
		print("after as func")
		owner:sub_mana(card:get_cost())
		print("after sub manae")
		battle1:save_ai(battle.ai)
		print("save ai")
		battle1:save_player(battle.player)
		print("save player")
		return function()
			local ownedRunes = owner:get_runes()
			print"after getting runes"
			for k, v in pairs(ownedRunes) do
				runes.run_rune_code(owner, k, v, "end_of_turn_effect", {v,battle1, owner, oponent})
				print"ran rune code"
			end
		end
	end
end

local function decideTurnOrder()
	local inOrder = {}
	local aiSpeed = battle.aiCard:get_speed()
	local playerSpeed = battle.playerCard:get_speed()

	print("before player", battle.playerSpeed)
	playerSpeed = maths.addPercentage(playerSpeed, runes.process_speed_runes(battle.player, battle.ai, battle.playerCard))
	print("after player", playerSpeed)
	aiSpeed = maths.addPercentage(aiSpeed, runes.process_speed_runes(battle.ai, battle.player, battle.aiCard))
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

	local aiCardAsFunc = createRunCardFunc(battle.aiCard, battle.battle, battle.ai, battle.player)
	local playerCardAsFunc = createRunCardFunc(battle.playerCard, battle.battle, battle.player, battle.ai)

	if aiSpeed > playerSpeed then
		table.insert(inOrder, aiCardAsFunc)
		table.insert(inOrder, playerCardAsFunc)
	else
		table.insert(inOrder, playerCardAsFunc)
		table.insert(inOrder, aiCardAsFunc)
	end
	return inOrder
end

local function doTurn()
	local cardsInOrder = decideTurnOrder()
	local atEndOfTurnRunes = {}
	for _, card in ipairs(cardsInOrder) do
		table.insert(atEndOfTurnRunes, card())
		if battle.battle:has_ended() then
			break
		end
	end

	for _, rune in ipairs(atEndOfTurnRunes) do
		rune()
		if battle.battle:has_ended() then
			break
		end
	end
	return battle.end_turn()
end

return doTurn