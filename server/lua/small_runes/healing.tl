local x : Rune.Rune = {
    turns_left= 3,
	end_of_turn_effect = function(self:Rune.Rune, config:Rust.SmallRune, battle:Rust.Field, owner:Rust.Player,oponent:Rust.Player):boolean
        config:dec_turns_left()
        owner:heal(10)
        return true
	end
}
return x