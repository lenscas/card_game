local constants = require"compiler/constants"
local files = require "compiler/fileSystem"
local saver = require"compiler/saver"


local function doStr(str)
	local func, err = load("return " .. str)
	if err == nil then
		return func()
	end
	error("Error in " .. str .. "Error :\n" .. err)
end


local function process_cards(fileName, asStr)
	local card = doStr(asStr)
	card.func = nil
	card.code = asStr
	saver.readySaveCard(card,fileName)
end

local function process_small_runes(fileName, asStr)
	local asCode = doStr(asStr)
	for key, value in pairs(asCode) do
		if type(value) == "function" then
			asCode[key] = nil
		end
	end
	files.writeToFile(constants.PATH_COMPILED_SMALL_RUNES, "config/" .. fileName, json.encode(asCode))
	files.writeToFile(constants.PATH_COMPILED_SMALL_RUNES, "code/" .. fileName, "return " .. asStr)
end

local function makeProcessFunction(func, rawPath)
	return function(cardPath)
		if not cardPath:match(".lua") then
			return nil
		end
		print(rawPath, cardPath)
		func(cardPath, files.readFull(rawPath, cardPath))
	end
end

local function process(func, rawPath)
	local procFunc = makeProcessFunction(func, rawPath)
	for v in files.getFilesInDir(rawPath) do
		procFunc(v)
	end
end

return {
	process = process,
	processCards = process_cards,
	processSmallRunes = process_small_runes
}