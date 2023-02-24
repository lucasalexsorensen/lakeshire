--[[
	modified based on http://lua-users.org/wiki/SaveTableToFile
	to serialize the Protobuf AST to file for future loading
]]
-- declare local variables
-- // exportstring( string )
-- // returns a "Lua" portable version of the string
local proto_parser = require("pb.proto.parser")

local function exportstring(s)
	return string.format("%q", s)
end

-- // The Save Function
local function save(tbl, pre, post)
	local charS, charE = "   ", "\n"
	-- local file, err = io.open(filename, "wb")
	local file, err = io.stdout
	if err then
		return err
	end

	-- initiate variables for save procedure
	local tables, lookup = { tbl }, { [tbl] = 1 }
	-- file:write( "return {"..charE )
	file:write("-- DO NOT EDIT THIS FILE, GENERATED BY LUAPB-SAVEAST " .. charE)
	file:write(pre .. charE)
	file:write("local t = {" .. charE)

	for idx, t in ipairs(tables) do
		file:write("-- Table: {" .. idx .. "}" .. charE)
		file:write("{" .. charE)
		local thandled = {}

		for i, v in ipairs(t) do
			thandled[i] = true
			local stype = type(v)
			-- only handle value
			if stype == "table" then
				if not lookup[v] then
					table.insert(tables, v)
					lookup[v] = #tables
				end
				file:write(charS .. "{" .. lookup[v] .. "}," .. charE)
			elseif stype == "string" then
				file:write(charS .. exportstring(v) .. "," .. charE)
			elseif stype == "number" then
				file:write(charS .. tostring(v) .. "," .. charE)
			end
		end

		for i, v in pairs(t) do
			-- escape handled values
			if not thandled[i] then
				local str = ""
				local stype = type(i)
				-- handle index
				if stype == "table" then
					if not lookup[i] then
						table.insert(tables, i)
						lookup[i] = #tables
					end
					str = charS .. "[{" .. lookup[i] .. "}]="
				elseif stype == "string" then
					str = charS .. "[" .. exportstring(i) .. "]="
				elseif stype == "number" then
					str = charS .. "[" .. tostring(i) .. "]="
				end

				if str ~= "" then
					stype = type(v)
					-- handle value
					if stype == "table" then
						if not lookup[v] then
							table.insert(tables, v)
							lookup[v] = #tables
						end
						file:write(str .. "{" .. lookup[v] .. "}," .. charE)
					elseif stype == "string" then
						file:write(str .. exportstring(v) .. "," .. charE)
					elseif stype == "number" then
						file:write(str .. tostring(v) .. "," .. charE)
					end
				end
			end
		end
		file:write("}," .. charE)
	end
	file:write("}")
	file:write(charE)

	file:write([=[
   local function loadast()
      local tables = t
      for idx = 1,#tables do
	 local tolinki = {}
	 for i,v in pairs( tables[idx] ) do
	    if type( v ) == "table" then
	       tables[idx][i] = tables[v[1]]
	    end
	    if type( i ) == "table" and tables[i[1]] then
	       table.insert( tolinki,{ i,tables[i[1]] } )
	    end
	 end
	 -- link indices
	 for _,v in ipairs( tolinki ) do
	    tables[idx][v[2]],tables[idx][v[1]] =  tables[idx][v[1]],nil
	 end
      end
      return tables[1]
   end
 ]=])
	file:write(charE)
	file:write(post)

	file:close()
end

-- print(proto_parser)
local text = io.stdin:read("*a")
-- print(text)

local ast = proto_parser.parse(text)

local varname = arg[1] or "pbast"

save(
	ast,
	[[
local _, ADDONSELF = ...
]],
	[[
ADDONSELF.]] .. varname .. [[ = loadast()
]]
)
