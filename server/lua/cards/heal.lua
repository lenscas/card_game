{
	id="test_heal",
	name = "some healing",
	description = "Heal 10",
	cost = 0,
	speed = 2,
	is_starting=true,
	func = function(params)
		params.owner:heal(10)
	end
}
