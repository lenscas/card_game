{
	id="test_heal",
	image="./button.png",
	name = "some healing",
	cost = 0,
	speed = 2,
	func = function(self, battle, owner,oponent)
		owner:heal(10)
	end
}
