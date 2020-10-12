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
		return function(event_list)
			local ownedRunes = owner:get_runes()
			for k, v in ipairs(ownedRunes) do
				if runes.run_rune_code(owner, k, v, "end_of_turn_effect", {v,owner, oponent}) then
					print(k,v)
					local a = event_list.create_trigger_small_rune_user(k);
					print(a)
					event_list:add_after(a)
				end
			end
		end
	end
end

local function decideTurnOrder()
	local aiSpeed = battle.aiCard:get_speed()
	local playerSpeed = battle.playerCard:get_speed()
	local raw_extra_speed_player, triggered_runes_player = runes.process_speed_runes(battle.player, battle.ai, battle.playerCard)
	local raw_extra_speed_ai, triggered_runes_ai = runes.process_speed_runes(battle.ai, battle.player, battle.aiCard)
	
	playerSpeed = maths.addPercentage(playerSpeed, raw_extra_speed_player)
	aiSpeed = maths.addPercentage(aiSpeed, raw_extra_speed_ai)
	
	local aiCardAsFunc = {
		func = createRunCardFunc(battle.aiCard, battle.ai, battle.player),
		triggered_runes =  triggered_runes_ai,
		card = battle.aiCard
	}
	local playerCardAsFunc = {
		func = createRunCardFunc(battle.playerCard, battle.player, battle.ai),
		triggered_runes = triggered_runes_player,
		card = battle.playerCard
	}

	local inOrder = {}

	--TODO : This if is to randomly select between the AI/Player if there speeds are equal
	--This is done SUPER UGLY!
	--FIX IT!

	if aiSpeed == playerSpeed then
		if math.random(0, 1) == 0 then
			aiSpeed = 0
			playerSpeed = 1
		else
			aiSpeed = 1
			playerSpeed = 0
		end
	end


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
	print(cardsInOrder[2].card:get_id())
	cardsInOrder[1].event = battle.battle.create_action_events(
		battle.battle.create_action(
			cardsInOrder[1].card:get_id()
		)
	)
	cardsInOrder[2].event= battle.battle.create_action_events(
		battle.battle.create_action(
			cardsInOrder[2].card:get_id()
		)
	)
	local event_list = battle.battle.create_event_list(cardsInOrder[1].event,cardsInOrder[2].event)
	for key, card in ipairs(cardsInOrder) do
		print("line 88", card.triggered_runes.owner)
		for _ , id in pairs(card.triggered_runes.owner) do
			print("line : 93",key)
			card.event:add_trigger_before(event_list.create_trigger_small_rune_user(id))
		end
		for _,id in ipairs(card.triggered_runes.hexa) do
			card.event:add_trigger_before(event_list.create_trigger_hexa_rune(id))
		end
		for _,id in ipairs(card.triggered_runes.oponent) do
			card.event:add_trigger_before(event_list.create_trigger_small_rune_oponent(id))
		end
		print("the key is: ", key)
		if key == 1 then
			print("got here?")
			event_list:add_first_action(card.event);
		else 
			event_list:add_second_action(card.event);
		end
		table.insert(atEndOfTurnRunes, card.func(event_list))
		if battle.battle:has_ended() then
			return true, event_list
		end
	end

	for _, rune in ipairs(atEndOfTurnRunes) do
		rune(event_list)
		if battle.battle:has_ended() then
			return true,event_list
		end
	end
	return battle.battle:has_ended(),event_list
end

local function doTurn()
	local has_ended,turn_events = procTurn()
	return battle.end_turn(),turn_events,has_ended
end

return doTurn