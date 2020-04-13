{
    turns_left= 3,
    end_of_turn_effect = function(self, config,battle,owner,oponent)
        config:dec_turns_left()
        oponent:deal_damage(5)
	end
}