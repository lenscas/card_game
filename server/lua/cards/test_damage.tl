return {
	id="test_damag",
	name = "deal damage",
	description="10 damage",
	cost = 0,
	speed = 2,
	is_starting=true,
	func = function(params : CardFuncParams)
		local battle = require"engine/battle"
		print("owner", params.owner)
		battle.deal_damage{
			amount = 3,
			from = params.owner,
			to = params.oponent,
			event_list = params.event_list,
			card_list = params.card_list
		}
	end
}
