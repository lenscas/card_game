global record RuneTypes

    enum RuneMethodNames
        "end_of_turn_effect"
        "owner_modify_damage"
        "owner_modify_speed"
        "oponent_modify_speed"
     end

     type Rune = {RuneMethodNames:function(Rune,any,any,any)}
     
     record ReturnValue
        value: number
        register_anyway : boolean
     end
     record TriggeredRunes 
         owner: {number}
         hexa: {number}
         oponent:{number}
      end
   

end

return RuneTypes