--This script turns the lua code for cards and runes into a format that is easier to work with.
--Every script needed to "compile" the cards are in the /compiler folder.

local processor = require "compiler/processor"
local constants = require "compiler/constants"
local saver = require "compiler/saver"

processor.process(processor.processCards, constants.PATH_RAW_CARDS)
processor.process(processor.processSmallRunes, constants.PATH_RAW_SMALL_RUNES)
processor.process(processor.processHexaRunes,constants.PATH_RAW_HEXA_RUNES)
saver.doSave()