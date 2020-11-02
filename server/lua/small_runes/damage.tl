local x : Rune.Rune = {
    turns_left= 3,
    owner_modify_damage = function(self:Rune.Rune, config:Rust.SmallRune, battle: Rust.Field,owner :Rust.Player,oponent: Rust.Player):number,boolean
        config:dec_turns_left()
        return 100
    end
    --[[
    oponent_modify_speed = function(self,config, card, oponent)
        print("now, lets modify the speed of our oponents card")
        return 0
    end,
    before_turn = function(self, card, owner, oponent)
        print("now we can deal damage over time, etc")
    end,
    before_dealing_damage = function(self, card, owner)
        print("now we can do something before a card we cast deals damage to our oponent")
        print("the number we return will be in adition to")
        return 0
    end,
    before_receiving_damage = function(self, card, oponent)
        print("now we can do something before a card deals damage")
    end,
    before_getting_healed = function(self, card, owner)
    end,
    before_oponent_heal = function(self, card, oponent)
    end
    before_casting =  function() print("I do something before casting") end,
    on_casting = function() print("and now when the player casts something") end,
    after_casting = function() print("and now after cast period is over") end,
    on_targeted = function() print("and I do something when my owner gets targeted") end,
    --]]
}
return x