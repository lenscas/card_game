{
	id="hexa_rune",
	name = "test hexa",
	description="+10 damage to all spells.",
	cost = 0,
	speed = 2,
	func = function(self, owner,oponent)
		local battle = require"engine/battle"
		battle.battle:add_rune("damage_buff")
	end
}
