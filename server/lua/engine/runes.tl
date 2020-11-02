local constants = require"engine/constants"

local function load_small_rune_code (rune : Rust.SmallRune): Rune.Rune
	local name = rune:get_name()
	return dofile(constants.SMALL_RUNE_BASE_FOLDER .. name .. ".lua") as Rune.Rune
end

local function load_hexa_rune_code(rune : Rust.HexaRune): Rune.Rune
	local name = rune:get_name()
	return dofile(constants.HEXA_RUNE_BASE_FOLDER .. name .. ".lua") as Rune.Rune
end

local function run_rune_code(
	battle: Rust.Field,
	owner: Rust.Player,
	index : number,
	rune : Rust.SmallRune, 
	rune_func : Rune.BasicRuneMethodNames,
	params : {any}
): (number,boolean)
	index = index - 1
	local code = load_small_rune_code(rune)
	local func = code[rune_func]
	if code[rune_func] then
		local x= code[rune_func] as function(...:any):(number,boolean) 
		local y = {x(code,table.unpack(params))}
		local ret_value = y
		owner:save_rune(rune,index)
		return table.unpack(ret_value) as (number,boolean)
	end
end

local function run_turn_trigger_rune(
	battle: Rust.Field,
	owner: Rust.Player,
	index : number,
	rune : Rust.SmallRune, 
	rune_func : Rune.TriggeredByTurnMethodNames,
	params : {any}
): boolean
	index = index - 1
	local code = load_small_rune_code(rune)
	local func = code[rune_func]
	if code[rune_func] then
		local x= code[rune_func] as function(...:any):boolean
		local y = x(code,table.unpack(params))
		owner:save_rune(rune,index)
		return y
	end
end

local function run_rune_speed_code(
	battle: Rust.Field,
	owner: Rust.Player,
	index : number,
	rune : Rust.SmallRune, 
	rune_func : Rune.SpeedRuneMethodNames,
	params : {any}
):(number,boolean)
index = index - 1
	local code = load_small_rune_code(rune)
	local func = code[rune_func]
	if code[rune_func] then
		local x= code[rune_func] as function(...:any):(number,boolean)
		local y = x(code,table.unpack(params))
		owner:save_rune(rune,index)
		return y
	end
end
local function run_hexa_code(
	battle: Rust.Field,
	index : number,
	rune: Rust.HexaRune, 
	rune_func : Rune.BasicHexaMethodNames, 
	params : {any}
): (number,boolean)
	index = index - 1
	local code = load_hexa_rune_code(rune)
	
	if code[rune_func] then
		local x= code[rune_func] as function(...:any):(number, boolean) 
		local y = x(code,table.unpack(params))
		local ret_value = {y}
		battle:save_rune(rune,index)
		return table.unpack(ret_value)
	end
end

local function run_turn_trigger_hexa_code(
	battle: Rust.Field,
	index : number,
	rune: Rust.HexaRune, 
	rune_func : Rune.TriggeredByTurnHexaMethodNames, 
	params : {any}
): boolean
	index = index - 1
	local code = load_hexa_rune_code(rune)
	
	if code[rune_func] then
		local x= code[rune_func] as function(...:any): boolean
		local y = x(code,table.unpack(params))
		battle:save_rune(rune,index)
		return y
	end
end



return {
	TriggeredRunes= {} as Rune.TriggeredRunes,
	run_hexa_code = run_hexa_code,
	run_rune_code = run_rune_code,
	run_turn_trigger_rune = run_turn_trigger_rune,
	process_speed_runes = function(
		battle: Rust.Field,
		caster:Rust.Player, 
		oponent : Rust.Player, 
		card: Rust.Card
	): (number,Rune.TriggeredRunes)

		local casterRunes = caster:get_runes()
		local extraSpeed = 0
		local triggered_runes: Rune.TriggeredRunes = {
			owner = {},
			hexa = {},
			oponent = {}
		}
		for k,v in ipairs(casterRunes) do
			print(k,v)
		end
		for k, v in ipairs(casterRunes) do
			print(k,v)
			local speed,register_anyway = run_rune_speed_code(
				battle,
				caster,
				k,
				v,
				"owner_modify_speed",
				{v, battle ,card, caster}
			)
			speed = speed or 0
			extraSpeed = extraSpeed + speed
			print(speed, register_anyway)
			if speed ~=0 or register_anyway then
				table.insert(triggered_runes.owner, k)
			end
		end
		local oponentRunes = oponent:get_runes()
		for k, v in ipairs(oponentRunes) do
			local speed,register_anyway = run_rune_speed_code(
				battle,
				caster,
				k,
				v,
				"oponent_modify_speed",
				{v, battle ,card, caster}
			)
			speed = speed or 0
			extraSpeed = extraSpeed - speed
			if extraSpeed ~=0 or register_anyway then
				table.insert(triggered_runes.oponent,k)
			end
		end
		return extraSpeed,triggered_runes
	end
}