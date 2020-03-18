{
    turns_left= 3,
    before_casting =  function() print("I do something before casting") end,
    on_casting = function() print("and now when the player casts something") end,
    after_casting = function() print("and now after cast period is over") end,
	on_targeted = function() print("and I do something when my owner gets targeted") end,
}