local constants = require"engine/constants"

local function load_small_rune_code (rune)
	local name = rune:get_name()
	return dofile(constants.SMALL_RUNE_BASE_FOLDER .. name .. ".lua")
end

local function load_hexa_rune_code(rune)
	local name = rune:get_name()
	return dofile(constants.HEXA_RUNE_BASE_FOLDER .. name .. ".lua")
end

local function run_rune_code(owner, index, rune, rune_func, params)
	index = index - 1
	local code = load_small_rune_code(rune)
	if code[rune_func] then
		local ret_value = {code[rune_func](code,table.unpack(params))}
		owner:save_rune(rune,index)
		return table.unpack(ret_value)
	end
end

local function run_hexa_code(battle,index,rune, rune_func, params)
	index = index - 1
	local code = load_hexa_rune_code(rune)
	if code[rune_func] then
		local ret_value = {code[rune_func](code,table.unpack(params))}
		battle:save_rune(rune,index)
		return table.unpack(ret_value)
	end
end

return {
	run_hexa_code = run_hexa_code,
	run_rune_code = run_rune_code,
	process_speed_runes = function(caster, oponent, card)
		local casterRunes = caster:get_runes()
		local extraSpeed = 0
		for k, v in pairs(casterRunes) do
			extraSpeed = extraSpeed + (run_rune_code(
					caster,
					k,
					v,
					"owner_modify_speed",
					{v, card, caster}
				) or 0)
		end
		local oponentRunes = oponent:get_runes()
		for k, v in pairs(oponentRunes) do
			extraSpeed = extraSpeed - (run_rune_code(
					caster,
					k,
					v,
					"oponent_modify_speed",
					{v, card, caster}
				) or 0)
		end
		return extraSpeed
	end
}