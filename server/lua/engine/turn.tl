local battle = require"engine/battle"
local maths = require"engine/math"
local runes = require"engine/runes"

local type EndOfTurnEffectsFunc = function(Rust.ActionsDuringTurn)
local type CardFunction = function(any,Rust.ActionsDuringTurn):EndOfTurnEffectsFunc

local function createRunCardFunc(card : Rust.Card, owner: Rust.Player, oponent:Rust.Player):CardFunction
	return function(card_list:Rust.Action, event_list:Rust.ActionsDuringTurn) :EndOfTurnEffectsFunc
		local code = card:get_code()
		print("card code:",code)
		local asFunc = load(code)() as CardTemplate
		
		asFunc.func{
			card = card,
			owner = owner,
			oponent = oponent,
			card_list = card_list,
			event_list = event_list
		}
	
		owner:sub_mana(card:get_cost())
		battle.battle:save_ai(battle.ai)
		battle.battle:save_player(battle.player)
		return function(event_list:Rust.ActionsDuringTurn)
			local ownedRunes = owner:get_runes()
			for k, v in ipairs(ownedRunes) do
				print("looping over runes", event_list)
				if runes.run_turn_trigger_rune(battle.battle,owner, k, v, "end_of_turn_effect", {v,battle.battle,owner, oponent} as {any}) then
					print(k,v)
					local a = event_list.create_trigger_small_rune_user(k);
					event_list:add_after(a)
				end
			end
		end
	end
end

local record ExecutableCard
	func:CardFunction
	triggered_runes:Rune.TriggeredRunes
	card:Rust.Card
	event:Rust.Action
end

local function decideTurnOrder():{ExecutableCard},boolean
	local aiSpeed = battle.aiCard:get_speed()
	local playerSpeed = battle.playerCard:get_speed()
	local raw_extra_speed_player, triggered_runes_player = runes.process_speed_runes(battle.battle,battle.player, battle.ai, battle.playerCard)
	local raw_extra_speed_ai, triggered_runes_ai = runes.process_speed_runes(battle.battle,battle.ai, battle.player, battle.aiCard)
	
	playerSpeed = maths.addPercentage(playerSpeed, raw_extra_speed_player)
	aiSpeed = maths.addPercentage(aiSpeed, raw_extra_speed_ai)
	
	local aiCardAsFunc:ExecutableCard = {
		func = createRunCardFunc(battle.aiCard, battle.ai, battle.player),
		triggered_runes =  triggered_runes_ai,
		card = battle.aiCard
	}
	local playerCardAsFunc:ExecutableCard = {
		func = createRunCardFunc(battle.playerCard, battle.player, battle.ai),
		triggered_runes = triggered_runes_player,
		card = battle.playerCard
	}

	local inOrder:{ExecutableCard} = {}

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

	local player_goes_first = false
	if aiSpeed > playerSpeed then
		table.insert(inOrder, aiCardAsFunc)
		table.insert(inOrder, playerCardAsFunc)
	else
		table.insert(inOrder, playerCardAsFunc)
		table.insert(inOrder, aiCardAsFunc)
		player_goes_first = true
	end
	return inOrder,player_goes_first
end

local function procTurn(): boolean, Rust.ActionsDuringTurn
	local cardsInOrder,player_goes_first = decideTurnOrder()
	local atEndOfTurnRunes:{EndOfTurnEffectsFunc} = {}
	
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
	local event_list = battle.battle.create_event_list(cardsInOrder[1].event,cardsInOrder[2].event,player_goes_first)
	for key, card in ipairs(cardsInOrder) do
		for _ , id in ipairs(card.triggered_runes.owner) do
			card.event:add_trigger_before(event_list.create_trigger_small_rune_user(id))
		end
		for _,id in ipairs(card.triggered_runes.hexa) do
			card.event:add_trigger_before(event_list.create_trigger_hexa_rune(id))
		end
		for _,id in ipairs(card.triggered_runes.oponent) do
			card.event:add_trigger_before(event_list.create_trigger_small_rune_defender(id))
		end
		table.insert(atEndOfTurnRunes, card.func(card.event,event_list))
		if key == 1 then
			event_list:add_first_action(card.event);
		else 
			event_list:add_second_action(card.event);
		end
		if battle.battle:has_ended() then
			return true, event_list
		end
	end

	for _, rune in ipairs(atEndOfTurnRunes) do
		print("original list", event_list)
		rune(event_list)
		if battle.battle:has_ended() then
			return true,event_list
		end
	end
	return battle.battle:has_ended(),event_list
end

local function doTurn():Rust.Field,Rust.ActionsDuringTurn,boolean
	local has_ended,turn_events = procTurn()
	return battle.end_turn(),turn_events,has_ended
end

return doTurn