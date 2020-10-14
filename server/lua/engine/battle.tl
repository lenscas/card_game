local battle = {
	battle = nil,
	chosenCard = nil,
	ai = nil,
	player = nil,
	aiCard = nil,
	aiCardIndex = nil,
	playerCard = nil,
	playerCardIndex = nil,
}

local runes = require"engine/runes"

--this function allows cards to deal damage and allow runes to interact with it
--the order of how damage interactions is calculated is
--runes from the caster that increase damage
--runes from the caster that decrease damage
--hexa runes that increase damage
--hexa runes that decrease damage
--runes from the target that increase damage
--runes from the target that decrease damage
local function deal_damage(params)
	print(params)
	local fromRunes = params.from:get_runes()
	--deal with the caster runes first
	local negatives = {}
	local amount = params.amount
	for k, rune in pairs(fromRunes) do
		local buff, register_anyway =
			runes.run_rune_code(
				from,
				k,
				rune,
				"owner_modify_damage",
				{rune, amount, params.from, params.to}
			) or 0
		if buff > 0 then
			amount = amount + (amount / 100 * buff)
		elseif buff < 0 then
			negatives[#negatives] = buff
		end
		if buff ~= 0 or register_anyway then
			params.card_list.event:add_trigger_before(params.event_list.create_trigger_small_rune_user(rune:id()))
		end
	end
	for _, v in ipairs(negatives) do
		amount = amount - (amount / 100 * v)
	end

	--now, time to deal with the hexa runes
	local hexaRunes = battle.battle:get_runes()
	negatives = {}
	for key, rune in ipairs(hexaRunes) do
		local value, register_anyway = runes.run_hexa_code(battle.battle,key,rune,"modify_damage",{rune,params.from,params.to})
		if value > 0 then
			amount = amount + (amount / 100 * value)
		elseif value <0 then
			table.insert(negatives,value)
		end
		if value ~= 0 or register_anyway then
			card.event:add_trigger_before(event_list.create_trigger_hexa_rune(rune:id()))
		end
	end
	for _,v in ipairs(negatives) do
		amount = amount - (amount / 100 * v)
	end

	--and finally, the runes from the target
	local positives = {}
	local toRunes = to:get_runes()
	for k, v in ipairs(toRunes) do
		local reduction, register_anyway = runes.run_rune_code(
				from,
				k,
				v,
				"receiver_modify_damage",
				{v, amount, params.from, params.to}
			) or 0
		if reduction > 0 then
			amount = amount - (amount / 100 * reduction)
		elseif reduction < 0 then
			positives[#positives] = reduction
		end
		if value ~= 0 or register_anyway then
			card.event:add_trigger_before(event_list.create_trigger_small_rune_defender(rune:id()))
		end
	end
	for _, v in pairs(positives) do
		amount = amount - (amount / 100 * v)
	end

	to:deal_damage(amount)
end

local function makeSaveRun(func)
	return function(...)
		assert(battle.battle, "a function in enviroment got called before it got initialized")
		local params = {...}
		print(params)
		print(table.unpack(params))
		return func(table.unpack(params))
	end
end

local function end_turn()
	battle.player:clean_up_runes()
	battle.ai:clean_up_runes()
	battle.player:add_mana(1)
	battle.ai:add_mana(1)
	battle.player:fill_hand()
	battle.ai:fill_hand()
	battle.battle:save_ai(battle.ai)
	battle.battle:save_player(battle.player)
	return battle.battle
end

local returnData = {
	deal_damage = makeSaveRun(deal_damage),
	end_turn = makeSaveRun(end_turn),
	init = function(self,battleInjected,chosenCardInjected)
		battle.battle = battleInjected
		battle.ai =  battleInjected:get_ai()
		battle.player =  battleInjected:get_player()
		battle.aiCard = battleInjected:get_ai_card()
		battle.playerCard = chosenCardInjected
		return self
	end
}

setmetatable(returnData, {__index = battle})

return returnData