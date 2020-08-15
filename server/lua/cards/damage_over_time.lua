{
	id="test_dop",
	name = "damage over time",
	description = "5 damage for 3 turns.",
	cost = 0,
	speed = 2,
	is_starting=true,
	func = function(self,owner,oponent)
		owner:add_rune("damage_over_time")
	end
}
