return {
    turns_left= 3,
    end_of_turn_effect = function(self, config,battle,first,second)
		second:deal_damage(5)
		first:deal_damage(5)
	end
}