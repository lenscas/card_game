{
	id="test_heal",
	name = "some healing",
	description = "Heal 10",
	cost = 0,
	speed = 2,
	is_starting=true,
	func = function(self, owner,oponent)
		owner:heal(10)
	end
}
