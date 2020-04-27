local json = require "json"
local sql = require("luasql.postgres").postgres()

local envReader = require "compiler/envReader"
local files = require"compiler/fileSystem"
local constants = require "compiler/constants"


local saved,con = {}, nil

local function getConnection()
	if con == nil then
		local conectionString = envReader("DATABASE_URL")
		con = assert(
			sql:connect(conectionString),
			"Couldn't connect to the database. Connection string used : " .. conectionString
		)
		--we don't want to automatically commit our changes, but make sure nothing wend wrong before we commit them all
		assert(con:setautocommit(false), "apparently psql does not have transactions? :(")
	end
	return con
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

		local con = getConnection()

		local textId = con:escape(cardId)
		local imagePath = con:escape(card.image)
		local isObtainable = tostring(
			card.is_obtainable == nil or (card.is_obtainable == true)
		)

		local as_json = json.encode(card)
		files.writeToFile(constants.PATH_COMPILED_CARDS, fileName, as_json)
		print(imagePath,fileName,textId,isObtainable, "end")

		local query = [[
INSERT INTO cards (
	image_path,
	json_file_path,
	text_id,
	is_obtainable
) VALUES (
	]] .. "'" ..imagePath.."','" ..
fileName.. "','" ..
	textId..
	"'," ..
	isObtainable .. [[
)
ON CONFLICT (text_id)
DO
	UPDATE
	SET
		image_path =  EXCLUDED.image_path,
		json_file_path = EXCLUDED.json_file_path,
		is_obtainable = EXCLUDED.is_obtainable
]]
		print(query)
		assert(con:execute(query))

		saved[textId] = fileName;
	end,
	--commits the changes to the database.
	doSave = function()
		local con = getConnection()
		con:commit()
	end
}
