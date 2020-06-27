{
	id="test_damag",
	name = "deal damage",
	description="10 damage",
	cost = 0,
	speed = 2,
	is_starting=true,
	func = function(self, owner,oponent)
		print("got here two?")
		local battle = require"engine/battle"
		print("got here?")
		battle.deal_damage(10,owner,oponent)
	end
}
