global record RuneTypes
    enum RuneMethodNames
        "end_of_turn_effect"
        "owner_modify_damage"
        "owner_modify_speed"
     end
     type Rune = {RuneMethodNames:function(Rune,any,any,any)}
     --[[
     record Rune
     end_of_turn_effect: function(Rune, any, any,any)
        owner_modify_damage:function(Rune, any, any,any)
        owner_modify_speed: function(Rune, any, any,any)
     end
     --]]
end
return RuneTypes