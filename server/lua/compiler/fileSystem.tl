--This module contain some functions to make it both easier to work with the file system
--and as a small abstraction over the lfs rock.
--It also prevents you from forgetting to close a file by not giving you direct access to file handlers.

local lfs = require "lfs"
local function readFullUsingPath(fullPath:string):string
	local handler = io.open(fullPath, "r")
	local contents = handler:read("*a")
	handler:close()
	return contents
end

local function writeToFileFullPath (path:string,str:string|number):string
	local handler, err = io.open(path, "w+")
	if err then
		print("path", path)
		error(err)
	end
	print("writing:")
	print(str)
	print()
	handler:write(tostring(str))
	handler:close()
	return path
end

return {
	--todo, only add the slash if it is needed.
	combinePathAndName = function(path:string,name:string):string
		return path .. "/" .. name
	end,
	--returns a list of every file inside a path
	getFilesInDir = function(path:string):function(...:any):string
		return lfs.dir(path)
	end,
	--same as readFull, but takes the entire path, rather than the filename seperatly
	readFullUsingPath = readFullUsingPath,

	-- Reads an entire file. NOTE: rawPath should NOT end with a slash
	readFull = function(rawPath:string, fileName:string):string
		local fullPath = rawPath .. "/" .. fileName
		return readFullUsingPath(fullPath)
	end,

	--Writes an string to a file.
	writeToFile = function(dir:string, fileName:string, str:string):string
		local path = dir .. "/" .. fileName
		return writeToFileFullPath(path,str)
		
	end,
	writeToFileFullPath = writeToFileFullPath,

	removeExtension = function(path:string):string
		return path:match "(.+)%..+"
	end,
	--run Func over every line inside a file
	openAndReadLines = function(path:string,func :function(string) )
		local file = io.open(path,"r")
		for line in file:lines() do
			func(line)
		end
		file:close()
	end
}