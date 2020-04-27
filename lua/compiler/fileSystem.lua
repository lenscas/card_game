local lfs = require "lfs"
return {
	getFilesInDir = function(path)
		return lfs.dir(path)
	end,

	readFull = function(rawPath, fileName)
		local fullPath = rawPath .. "/" .. fileName
		local handler = io.open(fullPath, "r")
		local cardAsStr = handler:read("*a")
		handler:close()
		return cardAsStr
	end,

	writeToFile = function(dir, fileName, str)
		local path = dir .. "/" .. fileName
		local handler, err = io.open(path, "w+")
		if err then
			print("path", path)
			error(err)
		end
		print("writing:")
		print(str)
		print()
		handler:write(str)
		handler:close()
		return path
	end,

	openAndReadLines = function(path,func)
		local file = io.open(path,"r")
		for line in file:lines() do
			func(line)
		end
		file:close()
	end
}