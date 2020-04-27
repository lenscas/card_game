{
	id="test_damag",
	image="./button.png",
	name = "deal damage",
	cost = 0,
	speed = 2,
	func = function(self, battle,owner,oponent)
		deal_damage(10,owner,oponent)
	end
}
