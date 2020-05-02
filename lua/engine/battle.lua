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

local function deal_damage(amount, from, to)
	local fromRunes = from:get_runes()
	local negatives = {}
	for k, v in pairs(fromRunes) do
		local buff =
			runes.run_rune_code(
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
		local reduction = runes.run_rune_code(
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

local function makeSaveRun(func)
	return function(...)
		print("????")
		assert(battle.battle, "an function in enviroment got called before it got initialized")
		local params = {...}
		return func(table.unpack(params))
	end
end

local function end_turn()
	battle.player:remove_card(battle.playerCardIndex)
	battle.ai:remove_card(battle.aiCardIndex)
	battle.player:clean_up_runes()
	battle.ai:clean_up_runes()
	battle.player:add_mana(1)
	battle.ai:add_mana(1)
	battle.battle:save_ai(battle.ai)
	battle.battle:save_player(battle.player)
	return battle.battle
end

local returnData = {
	deal_damage = makeSaveRun(deal_damage),
	end_turn = makeSaveRun(end_turn),
	init = function(self,battleInjected,chosenCardInjected)
		battle.chosenCard = chosenCardInjected
		battle.battle = battleInjected
		battle.ai =  battleInjected:get_ai()
		battle.player =  battleInjected:get_player()
		battle.aiCard, battle.aiCardIndex =  battleInjected:get_ai_card()
		battle.playerCard, battle.playerCardIndex =  battleInjected:get_player_card(chosenCardInjected)
		return self
	end
}

setmetatable(returnData, {__index = battle})

return returnData