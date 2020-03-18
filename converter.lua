--This script turns the lua code for cards and runes into a format that is easier to work with.

local PATH_RAW_CARDS = "./cards"
local PATH_COMPILED_CARDS = "./compiled_cards"

local PATH_RAW_SMALL_RUNES = "./small_runes"
local PATH_COMPILED_SMALL_RUNES = "./compiled_small_runes"

local lfs = require "lfs"
local json = require "json"

local function doStr(str)
	local func, err = load("return " .. str)
	if err == nil then
		return func()
	end
	error("Error in " .. str .. "Error :\n" .. err)
end

local function readFull(rawPath, fileName)
	local fullPath = rawPath .. "/" .. fileName
	local handler = io.open(fullPath, "r")
	local cardAsStr = handler:read("*a")
	handler:close()
	return cardAsStr
end

local function writeToFile(dir, fileName, str)
	handler = io.open(dir .. "/" .. fileName, "w+")
	print("writing:")
	print(str)
	print()
	handler:write(str)
	handler:close()
end

local function process_cards(fileName, asStr)
	local card = doStr(asStr)
	card.func = nil
	card.code = asStr
	local as_json = json.encode(card)
	writeToFile(PATH_COMPILED_CARDS, fileName, as_json)
end

local function process_small_runes(fileName, asStr)
	local asCode = doStr(asStr)
	for key, value in pairs(asCode) do
		if type(value) == "function" then
			asCode[key] = nil
		end
	end
	writeToFile(PATH_COMPILED_SMALL_RUNES, "config/" .. fileName, json.encode(asCode))
	writeToFile(PATH_COMPILED_SMALL_RUNES, "code/" .. fileName, asStr)
end

local function makeProcessFunction(func, rawPath)
	return function(cardPath)
		if not cardPath:match(".lua") then
			return nil
		end
		print(rawPath, cardPath)
		func(cardPath, readFull(rawPath, cardPath))
	end
end

local function process(func, RAW_PATH)
	local procFunc = makeProcessFunction(func, RAW_PATH)
	for v in lfs.dir(RAW_PATH) do
		procFunc(v)
	end
end

process(process_cards, PATH_RAW_CARDS)
process(process_small_runes, PATH_RAW_SMALL_RUNES)
