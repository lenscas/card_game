local battle = require"engine/battle"
local maths = require"engine/math"
local runes = require"engine/runes"

local function createRunCardFunc(card, owner, oponent)
	return function()
		local code = card:get_code()
		local asFunc = load("return " .. code)()
		asFunc.func(card, owner, oponent)
		owner:sub_mana(card:get_cost())
		battle.battle:save_ai(battle.ai)
		battle.battle:save_player(battle.player)
		return function()
			local ownedRunes = owner:get_runes()
			for k, v in pairs(ownedRunes) do
				runes.run_rune_code(owner, k, v, "end_of_turn_effect", {v,owner, oponent})
			end
		end
	end
end

local function decideTurnOrder()
	local inOrder = {}
	local aiSpeed = battle.aiCard:get_speed()
	local playerSpeed = battle.playerCard:get_speed()

	playerSpeed = maths.addPercentage(playerSpeed, runes.process_speed_runes(battle.player, battle.ai, battle.playerCard))
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

	local aiCardAsFunc = {
		func = createRunCardFunc(battle.aiCard, battle.ai, battle.player), 
		card = battle.aiCard
	}
	local playerCardAsFunc = {
		func = createRunCardFunc(battle.playerCard, battle.player, battle.ai),
		card = battle.aiCard
	}


	if aiSpeed > playerSpeed then
		table.insert(inOrder, aiCardAsFunc)
		table.insert(inOrder, playerCardAsFunc)
	else
		table.insert(inOrder, playerCardAsFunc)
		table.insert(inOrder, aiCardAsFunc)
	end
	return inOrder
end

local function procTurn()
	local cardsInOrder = decideTurnOrder()
	local atEndOfTurnRunes = {}
	print(cardsInOrder[1].card:get_id())
	local event_list = battle.battle.create_event_list(
		battle.battle.create_action_events(
			battle.battle.create_action(
				cardsInOrder[1].card:get_id()
			)
		),
		battle.battle.create_action_events(
			battle.battle.create_action(
				cardsInOrder[2].card:get_id()
			)
		)	
	)
	for _, card in ipairs(cardsInOrder) do
		table.insert(atEndOfTurnRunes, card.func())
		if battle.battle:has_ended() then
			return true
		end
	end

	for _, rune in ipairs(atEndOfTurnRunes) do
		rune()
		if battle.battle:has_ended() then
			return true
		end
	end
	return battle.battle:has_ended(),event_list
end

local function doTurn()
	local has_ended,turn_events = procTurn()
	return battle.end_turn(),turn_events,has_ended
end

return doTurn