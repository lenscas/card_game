{
	id="test_runes",
	image="./button.png",
	name = "add speed rune",
	cost = 0,
	speed = 2,
	func = function(self, battle,owner,oponent)
		owner:add_rune("speed")
		owner:add_rune("damage")
	end
}
