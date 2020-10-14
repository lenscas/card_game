{
	id="hexa_rune",
	name = "test hexa",
	description="+10 damage to all spells.",
	cost = 20,
	speed = 2,
	func = function(params)
		local battle = require"engine/battle"
		battle.battle:add_rune("damage_buff")
	end
}
