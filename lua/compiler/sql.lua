--This contains functions to work with the database.

--------------------------------------------------------------
--NOTE: This is the ONLY place where sql queries should exist.
--------------------------------------------------------------

local sql = require("luasql.postgres").postgres()

local envReader = require "compiler/envReader"

local con = nil

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
	saveCard = function(card,fileName)
		local con = getConnection()
		local textId = con:escape(card.id)
		local isObtainable = tostring(
			card.is_obtainable == nil or (card.is_obtainable == true)
		)
		print(fileName,textId,isObtainable)
		local query = [[
INSERT INTO cards (
	json_file_path,
	text_id,
	is_obtainable
) VALUES (
	]] .. "'" ..fileName.."','" ..
textId.. "','" ..
	isObtainable..
[['
)
ON CONFLICT (text_id)
DO
	UPDATE
	SET
		json_file_path = EXCLUDED.json_file_path,
		is_obtainable = EXCLUDED.is_obtainable
]]
	assert(con:execute(query))
	end,
	commit = function()
		local con = getConnection()
		con:commit()
	end
}