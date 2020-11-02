--This module is to save the new format for the cards at their correct places in the file system

local json = require "json"

local imageCreator = require "compiler/imageCreator"
local sql = require"compiler/sql"
local files = require"compiler/fileSystem"
local constants = require "compiler/constants"


local saved : {string:string} = {}

local function saveRune(basePath:string,fileName:string, rune: Rune.Rune, codeStr:string)
	fileName = files.removeExtension(fileName)
	files.writeToFile(basePath, "config/" .. fileName..".json", json.encode(rune))
	files.writeToFile(basePath, "code/" .. fileName..".lua", codeStr)
end

return {
	--this function saves it to disk and also does the upsert in the database.
	--HOWEVER it does not yet commit the changes to the database.
	--To commit, see doSave

	--this means that if lua errors, the files and the database ARE NOT in sync any longer.
	readySaveCard = function(card: CardTemplate, fileName:string)
		fileName = files.removeExtension(fileName)..".json"
		local cardId = card.id
		assert(cardId, "No card Id found in: " .. fileName)
		assert(not saved[cardId],
				cardId .. [[ was already saved.
There is a conflict in the new set of cards.
Tried saving : ]] .. fileName .. [[
Collides with : ]] .. (saved[cardId] or "no collision")
		)

		sql.saveCard(card,fileName)
		imageCreator.imageifyCard(card)
		local as_json = json.encode(card)
		files.writeToFile(constants.PATH_COMPILED_CARDS, fileName, as_json)
		


		saved[cardId] = fileName;
	end,
	saveSmallRune = function(fileName:string,rune:Rune.Rune,codeStr:string)
		saveRune(constants.PATH_COMPILED_SMALL_RUNES,fileName,rune,codeStr)
	end,
	saveHexaRune = function(fileName:string,rune:Rune.Rune,codeStr:string)
		saveRune(constants.PATH_COMPILED_HEXA_RUNES,fileName,rune,codeStr)
	end,
	--commits the changes to the database.
	doSave = function()
		files.writeToFileFullPath(constants.FULL_PATH_CARD_GENERATION_TIME_ID_FILE,os.time())
		sql.commit()
	end
}
