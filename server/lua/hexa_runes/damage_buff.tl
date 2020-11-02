local x = {
    turns_left= 3,
    modify_damage : Rune.RuneFunc = function(self:Rune.Rune, config:{any:any},battle:Rust.Field,caster: Rust.Player,target:Rust.Player):number,boolean
		  return 10
	  end
}
return x