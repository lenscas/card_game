--This module creates the various images based upon a simple svg template.
--It stores both a .svg and .png version of images.

local constants = require "compiler/constants"
local files = require "compiler/fileSystem"

--we load the template lazily and cache it
local template;

local function escape (spec)
	local x = tostring(spec):gsub('[%-%.%+%[%]%(%)%^%%%?%*]','%%%1'):gsub('%$%%(%S)','%$%1')
	return x
end
local function escape2 (spec)
	local x = tostring(spec):gsub('%%','%%%%')
	return x
end

return {
	imageifyCard = function(card)
		assert(card.name,"The card " .. card.id .. " does not have a name")
		assert(card.description,"The card " .. card.id .. " does not have a description")
		assert(card.cost,"The card " .. card.id .. " does not have a cost")
		assert(card.speed,"The card " .. card.id .. " does not have a speed")

		if not template then
			template = files.readFullUsingPath(constants.PATH_BASIC_CARD_FRAME)
		end
		local v = constants.CARD_FRAME_REPLACEMENTS

		print("start new card")
		print("done", escape(v.DESCRIPTION),escape2(card.description))
		--os.exit()
		template:gsub(escape(v.DESCRIPTION), escape2(card.description))


		local newSvg = template:gsub(escape(v.NAME),escape2(card.name))
			:gsub(escape(v.DESCRIPTION), escape2(card.description))
			:gsub(escape(v.SPEED),escape2(card.speed))
			:gsub(escape(v.MANA),escape2(card.cost))
		local fullPath = files.writeToFile(constants.PATH_OUT_CARD_PICTURES, card.id .. ".svg", newSvg)
		local path_png = constants.PATH_OUT_CARD_PICTURES  .. "/".. card.id .. ".png"
		os.execute("inkscape --export-background=#FFFFFF --export-png="..path_png .. " " .. fullPath)
	end
}
