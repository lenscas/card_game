global record Rune
    enum BasicRuneMethodNames
        "owner_modify_damage"
        "receiver_modify_damage"
    end
    enum SpeedRuneMethodNames
        "owner_modify_speed"
        "oponent_modify_speed"
    end
    enum TriggeredByTurnMethodNames
        "end_of_turn_effect"
    end
    enum RuneMethodNames
        "owner_modify_speed"
        "oponent_modify_speed"
        "owner_modify_damage"
        "receiver_modify_damage"
        "end_of_turn_effect"
    end
    enum HexaMethodNames
        "end_of_turn_effect"
        "modify_damage"
    end
    enum BasicHexaMethodNames
        "modify_damage"
    end
    enum TriggeredByTurnHexaMethodNames
        "end_of_turn_effect"
    end
    type BasicRuneFunc = function(Rune,Rust.SmallRune,Rust.Field, Rust.Player,Rust.Player):(integer,boolean)
    type RuneSpeedFunc = function(Rune,Rust.SmallRune,Rust.Card,Rust.Player):(integer,boolean)
    type RuneFuncTurnTrigger = function(Rune,Rust.SmallRune,Rust.Field, Rust.Player,Rust.Player):boolean
    type HexaRuneFunc = function(Rune,Rust.HexaRune,Rust.Field, Rust.Player,Rust.Player):boolean
    record Rune
        turns_left: integer
        end_of_turn_effect: RuneFuncTurnTrigger
        modify_damage: BasicRuneFunc
        owner_modify_damage: BasicRuneFunc
        owner_modify_speed: RuneSpeedFunc
        oponent_modify_speed : RuneSpeedFunc
        receiver_modify_damage: BasicRuneFunc
    end
    record HexaRune
        turns_left: integer
        end_of_turn_effect: HexaRuneFunc
        modify_damage: BasicRuneFunc
    end
     
    record ReturnValue
        value: integer
        register_anyway : boolean
    end
    record TriggeredRunes 
        owner: {integer}
        hexa: {integer}
        oponent:{integer}
    end
end

return Rune