{
	id="test_damag",
	image="./button.png",
	name = "deal damage",
	cost = 0,
	speed = 2,
	func = function(self, owner,oponent)
		print("got here two?")
		local battle = require"engine/battle"
		print("got here?")
		battle.deal_damage(10,owner,oponent)
	end
}
