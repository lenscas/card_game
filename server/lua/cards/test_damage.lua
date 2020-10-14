{
	id="test_damag",
	name = "deal damage",
	description="10 damage",
	cost = 0,
	speed = 2,
	is_starting=true,
	func = function(params)
		local battle = require"engine/battle"
		battle.deal_damage{
			amount = 3,
			owner = params.owner,
			oponent = params.oponent,
			event_list = params.event_list,
			card_list = params.card_list
		}
	end
}
