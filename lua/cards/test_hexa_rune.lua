{
	id="hexa_rune",
	image="./button.png",
	name = "test hexa",
	cost = 0,
	speed = 2,
	func = function(self, owner,oponent)
		local battle = require"engine/battle"
		battle.battle:add_rune("damage_buff")
	end
}
