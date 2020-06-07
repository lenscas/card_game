{
	id="test_runes",
	name = "add speed rune",
	description = "+10 to speed for 3 spells.\n100% extra damage for 3 spells.",
	cost = 0,
	speed = 2,
	func = function(self, owner,oponent)
		owner:add_rune("speed")
		owner:add_rune("damage")
	end
}
