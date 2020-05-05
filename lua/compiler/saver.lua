local json = require "json"
local sql = require"compiler/sql"

local files = require"compiler/fileSystem"
local constants = require "compiler/constants"


local saved = {}

local function saveRune(basePath,fileName, rune, codeStr)
	files.writeToFile(basePath, "config/" .. fileName, json.encode(rune))
	files.writeToFile(basePath, "code/" .. fileName, "return " .. codeStr)
end

return {
	--this function saves it to disk and also does the upsert in the database.
	--HOWEVER it does not yet commit the changes to the database.
	--To commit, see doSave

	--this means that if lua errors, the files and the database ARE NOT in sync any longer.
	readySaveCard = function(card, fileName)
		local cardId = card.id
		assert(cardId, "No card Id found in: " .. fileName)
		assert(not saved[cardId],
				cardId .. [[ was already saved.
There is a conflict in the new set of cards.
Tried saving : ]] .. fileName .. [[
Collides with : ]] .. (saved[cardId] or "no collision")
		)

		sql.saveCard(card,fileName)
		local as_json = json.encode(card)
		files.writeToFile(constants.PATH_COMPILED_CARDS, fileName, as_json)


		saved[cardId] = fileName;
	end,
	saveSmallRune = function(fileName,rune,codeStr)
		saveRune(constants.PATH_COMPILED_SMALL_RUNES,fileName,rune,codeStr)
	end,
	saveHexaRune = function(fileName,rune,codeStr)
		saveRune(constants.PATH_COMPILED_HEXA_RUNES,fileName,rune,codeStr)
	end,
	--commits the changes to the database.
	doSave = function()
		sql.commit()
	end
}
