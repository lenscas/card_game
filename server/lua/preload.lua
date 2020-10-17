--this file is a wrapper around the engine.tl file
--it loads the compiler so tl files can be loaded by rlua directly
--it is also a way to get arround https://github.com/teal-language/tl/issues/240

--this file will be obsolite once tealr allows preloading the compiler and that bug is fixed
package.path = './lua/?.lua;' .. package.path

local tl = require("teal")
print("load teal")
tl.loader()
print("enable teal")
local engine = require"engine"
print("load engine")
return engine()