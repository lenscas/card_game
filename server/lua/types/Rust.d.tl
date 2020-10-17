local record RustTypes
    record Field
        -- pure Methods
        get_ai: function(Field):(Player)
        get_player: function(Field):(Player)
        has_ended: function(Field):(boolean)
        get_runes: function(Field):({HexaRune})
        -- Mutating Methods
        get_ai_card: function(Field):(Card)
        save_ai: function(Field, Player):()
        save_player: function(Field, Player):()
        save_rune: function(Field, HexaRune, number):()
        clean_up_runes: function(Field):()
        add_rune: function(Field, string):()
        -- Pure functions
        create_action: function(string):(PossibleActions)
        create_action_events: function(PossibleActions):(Action)
        create_event_list: function(Action, Action):(ActionsDuringTurn)

    end
    record Card
        -- pure Methods
        get_cost: function(Card):(number)
        get_speed: function(Card):(number)
        get_name: function(Card):(string)
        get_code: function(Card):(string)
        get_id: function(Card):(string)

    end
    record Player
        -- pure Methods
        get_mana: function(Player):(number)
        get_runes: function(Player):{SmallRune}
        -- Mutating Methods
        fill_hand: function(Player):()
        discard_cards: function(Player, number):()
        add_mana: function(Player, number):()
        sub_mana: function(Player, number):(boolean)
        deal_damage: function(Player, number):()
        heal: function(Player, number):()
        save_rune: function(Player, SmallRune, number):()
        clean_up_runes: function(Player):()
        add_rune: function(Player, string):()

    end
    record HexaRune
        -- pure Methods
        get_name: function(HexaRune):(string)
        get_turns_left: function(HexaRune):(number)
        id: function(HexaRune):(number)
        -- Mutating Methods
        dec_turns_left: function(HexaRune):(number)

    end
    record SmallRune
        -- pure Methods
        get_name: function(SmallRune):(string)
        get_turns_left: function(SmallRune):(number)
        id: function(SmallRune):(number)
        -- Mutating Methods
        dec_turns_left: function(SmallRune):(number)

    end
    record Action
        -- Mutating Methods
        add_trigger_before: function(Action, TriggerTypes):()
        add_trigger_after: function(Action, TriggerTypes):()

    end
    record ActionsDuringTurn
        -- Mutating Methods
        add_before: function(ActionsDuringTurn, TriggerTypes):()
        add_after: function(ActionsDuringTurn, TriggerTypes):()
        add_first_action: function(ActionsDuringTurn, Action):()
        add_second_action: function(ActionsDuringTurn, Action):()
        -- Pure functions
        create_trigger_small_rune_user: function(number):(TriggerTypes)
        create_trigger_hexa_rune: function(number):(TriggerTypes)
        create_trigger_small_rune_defender: function(number):(TriggerTypes)

    end
    record PossibleActions

    end
    record TriggerTypes

    end
end
return RustTypes