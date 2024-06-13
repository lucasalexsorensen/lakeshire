local ADDONNAME, ls = ...

local luapb = ls.luapb
local proto = luapb.load_proto_ast(ls.pbast).GameState
ls.testproto = proto()

ls.commands = {
	["help"] = function()
		print("Help goes here..")
	end,
	["start"] = function()
		print("Start!")
		ls.testproto.BotState = 1
	end,
	["stop"] = function()
		print("Stop!")
		ls.testproto.BotState = 0
	end,
	["toggle"] = function()
		if ls.testproto.BotState == "Stopped" then
			ls.testproto.BotState = "Running"
		else
			ls.testproto.BotState = "Stopped"
		end
		print("New state: " .. ls.testproto.BotState)
	end,
	["dump"] = function()
		print("Dumping position!")
		ls.testproto.BotState = 2

		C_Timer.After(1, ls.commands["stop"])
	end,
}

function ls:HandleSlashCommands(str)
	if #str == 0 then
		ls.commands.help()
		return
	end

	local args = {}
	for _, arg in ipairs({ string.split(" ", str) }) do
		if #arg > 0 then
			table.insert(args, arg)
		end
	end

	local path = ls.commands -- required for updating found table.
	for id, arg in ipairs(args) do
		if #arg > 0 then -- if string length is greater than 0.
			arg = arg:lower()
			if path[arg] then
				if type(path[arg]) == "function" then
					-- all remaining args passed to our function!
					path[arg](select(id + 1, unpack(args)))
					return
				elseif type(path[arg]) == "table" then
					path = path[arg] -- another sub-table found!
				end
			end
		end
	end
end

function ls:InitializeState(name)
	if name == ADDONNAME then
		ls.InitializeBotState()
		ls:InitializePlayer()

		ls.pixelinfo:UpdatePB(ls.testproto)

		SLASH_Lakeshire1 = "/ls"
		SlashCmdList.Lakeshire = function(...)
			ls:HandleSlashCommands(...)
		end

		ls.events:SetScript("OnUpdate", function()
			ls:HandleOnUpdate()
		end)
	end
end

function ls:InitializeBotState()
	ls.testproto.BotState = "Stopped"
end

function ls:InitializePlayer()
	ls.testproto.Player = { UnitInfo = {}, PosInfo = {} }
	ls.testproto.Player.UnitInfo.Name = UnitName("player")
	ls.testproto.Player.UnitInfo.Level = UnitLevel("player")
	local _, _, c = UnitClass("player")
	ls.testproto.Player.UnitInfo.Class = c
	ls.testproto.Player.UnitInfo.HealthCurrent = UnitHealth("player")
	ls.testproto.Player.UnitInfo.HealthMax = UnitHealthMax("player")
	ls.testproto.Player.UnitInfo.PowerCurrent = UnitPower("player")
	ls.testproto.Player.UnitInfo.PowerMax = UnitPowerMax("player")

	-- Pos Info
	ls:UpdatePosInfo()

	-- Flags
	ls:UpdatePlayerFlags()
end

function ls:UpdatePosInfo()
	local map = C_Map.GetBestMapForUnit("player")
	local position = C_Map.GetPlayerMapPosition(map, "player")
	local mapx, mapy = position:GetXY()
	local instanceid, world_position = C_Map.GetWorldPosFromMapPos(map, position)
	local y, x = world_position:GetXY()
	if x == nil or y == nil then
		x = 0
		y = 0
	end
	-- local p = Minimap                   -- not sure if needed
	-- local m = ({p:GetChildren()})[9]    -- also not sure if needed
	local rot = GetPlayerFacing()
	ls.testproto.Player.PosInfo = {
		MapX = math.ceil(mapx * 1e14),
		MapY = math.ceil(mapy * 1e14),
		MapId = map,
		WorldX = math.ceil(x * 1e5),
		WorldY = math.ceil(y * 1e5),
		InstanceId = instanceid,
		Facing = rot * 1e10,
	}
end

function ls:UpdatePlayerFlags()
	ls.testproto.Player.Flags = (IsOutdoors() and 1 or 0) + (IsMounted() and 2 or 0) + (IsFlying() and 4 or 0)
end

ls:Subscribe()
