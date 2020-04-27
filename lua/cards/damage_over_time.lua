{
	id="test_dop",
	image="./button.png",
	name = "damage over time",
	cost = 0,
	speed = 2,
	func = function(self, battle,owner,oponent)
		owner:add_rune("damage_over_time")
	end
}
