{
    turns_left= 3,
	end_of_turn = function(self, config, owner,oponent)
        config:dec_turns_left()
        owner:heal(10)
	end
}