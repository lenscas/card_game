return {
	id="test_runes",
	name = "add speed rune",
	description = "+10 to speed for 3 spells.\n100% extra damage for 3 spells.",
	cost = 0,
	speed = 2,
	is_starting=true,
	func = function(params : CardFuncParams)
		params.owner:add_rune("speed")
		params.owner:add_rune("damage")
	end
}
