local files = require "compiler/fileSystem"

local vars = {}

return function(key, allowError)
	allowError = allowError or false
	if next(vars) == nil then
		files.openAndReadLines("../.env",function(line)
			print(line)
			if line:sub(1,1) ~= "#" then
				local value = {}
				local varKey = {}
				local foundEqual = false
				for c in string.gmatch(line,".") do
					if c == "=" and not foundEqual then
						foundEqual = true
					elseif foundEqual then
						table.insert(value,c)
					else
						table.insert(varKey,c)
					end
				end
				value = table.concat(value,"")
				varKey = table.concat(varKey,"")
				vars[varKey] = value
			end
		end)
	end
	if vars[key] == nil and not allowError then
		local keys = {}
		for k,v in pairs(vars) do
			table.insert(keys,"'"..k.."'")
		end
		keys = table.concat(keys," , ")
		error("Could not load key : '"..key.. "' Available keys : "..keys)
	end
	return vars[key]
end
