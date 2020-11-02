--this contains the functions that actually do the heavy work of transforming the files from one format to another.

local files = require "compiler/fileSystem"
local saver = require"compiler/saver"

local function doStr<X>(str:string):X
	local func, err = load(str)
	if err == nil then
		return func() as X
	end
	error("Error in " .. str .. "Error :\n" .. err)
end


local function process_cards(fileName:string, asStr:string)
	local card: CardTemplate = doStr(asStr) as CardTemplate
	card.func = nil
	card.code = asStr
	saver.readySaveCard(card,fileName)
end

local function process_rune(asStr: string):Rune.Rune
	local asCode = doStr(asStr) as {any:any}
	for key, value in pairs(asCode) do
		if type(value) == "function" then
			asCode[key] = nil
		end
	end
	return asCode as Rune.Rune
end

local function process_small_runes(fileName:string, asStr:string)
	local asCode = process_rune(asStr)
	saver.saveSmallRune(fileName,asCode, asStr)
end

local function process_hexa_runes(fileName:string,asStr:string)
	local asCode = process_rune(asStr)
	saver.saveHexaRune(fileName,asCode, asStr)
end

local function makeProcessFunction(func:function(string,string), rawPath:string):function(string)
	return function(cardPath:string)
		if not cardPath:match(".tl") then
			return
		end
		print(rawPath, cardPath)
		local tempFile = os.tmpname()
		print(tempFile)
		os.execute("tl gen "..files.combinePathAndName(rawPath,cardPath).. " -o "..tempFile)
		local compiled_text = files.readFullUsingPath(tempFile)
		os.remove(tempFile)
		func(cardPath, compiled_text)
	end
end

local function process(func:function(string,string), rawPath:string)
	local procFunc = makeProcessFunction(func, rawPath)
	for v in files.getFilesInDir(rawPath) do
		procFunc(v)
	end
end

return {
	process = process,
	processCards = process_cards,
	processSmallRunes = process_small_runes,
	processHexaRunes = process_hexa_runes,
}