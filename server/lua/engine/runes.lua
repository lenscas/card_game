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
		local triggered_runes = {
			owner = {},
			hexa = {},
			oponent = {}
		}
		--[[
		for k,v in pairs(casterRunes) do
			print(k,v)
		end
		--]]
		for k, v in ipairs(casterRunes) do
			print(k,v)
			local speed, register_anyway = run_rune_code(
				caster,
				k,
				v,
				"owner_modify_speed",
				{v, card, caster}
			)
			speed = speed or 0
			extraSpeed = extraSpeed + speed
			print(speed, register_anyway)
			if speed ~=0 or register_anyway then
				print("got here????")
				table.insert(triggered_runes.owner, k)
			end
		end
		local oponentRunes = oponent:get_runes()
		for k, v in ipairs(oponentRunes) do
			local speed, register_anyway = run_rune_code(
				caster,
				k,
				v,
				"oponent_modify_speed",
				{v, card, caster}
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