{
	id="test_damag",
	image="./button.png",
	name = "deal damage",
	cost = 0,
	speed = 2,
	func = function(self, battle,owner,oponent)
		print("got here two?")
		local env = require"engine/battle"
		print("got here?")
		env.deal_damage(10,owner,oponent)
	end
}
