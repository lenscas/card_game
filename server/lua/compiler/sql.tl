--This contains functions to work with the database.

--------------------------------------------------------------
--NOTE: This is the ONLY place where sql queries should exist.
--------------------------------------------------------------

local sql = require("luasql.postgres").postgres()

local envReader = require "compiler/envReader"

local con :Sql.Connection = nil

local function getConnection():Sql.Connection
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
	saveCard = function(card:CardTemplate,fileName:string)
		local con = getConnection()
		local textId = con:escape(card.id)
		local isObtainable = tostring(
			card.is_obtainable == nil or (card.is_obtainable == true)
		)
		local is_starting = tostring(card.is_starting == true)
		print(fileName,textId,isObtainable)
		local query = [[
INSERT INTO cards (
	json_file_path,
	text_id,
	is_obtainable,
	is_starting_card
) VALUES (
	]] .. "'" ..
	fileName.."','" ..
	textId.. "','" ..
	isObtainable.. "','" ..
	is_starting ..

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