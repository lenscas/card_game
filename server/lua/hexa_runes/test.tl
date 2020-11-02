return {
    turns_left= 3,
    end_of_turn_effect: Rune.RuneFunc = function(self:Rune.Rune, config:{any:any},battle:Rust.Field,first:Rust.Player,second:Rust.Player): number,boolean
		second:deal_damage(5)
		first:deal_damage(5)
	end
}