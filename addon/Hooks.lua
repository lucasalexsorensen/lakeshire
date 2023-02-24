local ADDONNAME, ls = ...

function ls:HandleUnitHealth(...)
	local args = { ... }
	local unit = args[1]
	if unit == "player" then
		ls.testproto.Player.UnitInfo.HealthCurrent = UnitHealth("player")
		ls.testproto.Player.UnitInfo.HealthMax = UnitHealthMax("player")
		ls.pixelinfo:UpdatePB(ls.testproto)
	elseif unit == "target" then
		ls.testproto.Target.UnitInfo.HealthCurrent = UnitHealth("target")
		ls.testproto.Target.UnitInfo.HealthMax = UnitHealthMax("target")
		ls.pixelinfo:UpdatePB(ls.testproto)
	end
end

function ls:HandleUnitPower(...)
	local args = { ... }
	local unit = args[1]
	if unit == "player" then
		ls.testproto.Player.UnitInfo.PowerCurrent = UnitPower("player")
		ls.testproto.Player.UnitInfo.PowerMax = UnitPowerMax("player")
		ls.pixelinfo:UpdatePB(ls.testproto)
	end
end

function ls:HandleTargetChanged()
	if UnitExists("target") then
		ls.testproto.Target = { UnitInfo = {} }
		ls.testproto.Target.UnitInfo.Name = UnitName("target")
		ls.testproto.Target.UnitInfo.Level = UnitLevel("target")
		ls.testproto.Target.UnitInfo.HealthCurrent = UnitHealth("target")
		ls.testproto.Target.UnitInfo.HealthMax = UnitHealthMax("target")
		ls.pixelinfo:UpdatePB(ls.testproto)
	else
		ls.testproto.Target = nil
		ls.pixelinfo:UpdatePB(ls.testproto)
	end
end

function ls:HandleOnUpdate()
	ls:UpdatePosInfo()
	ls.pixelinfo:UpdatePB(ls.testproto)
end

ls.hooks = {
	ADDON_LOADED = {
		handler = function(...)
			ls:InitializeState(...)
		end,
	},
	UNIT_HEALTH = {
		handler = function(...)
			ls:HandleUnitHealth(...)
		end,
	},
	UNIT_POWER_UPDATE = {
		handler = function(...)
			ls:HandleUnitPower(...)
		end,
	},
	PLAYER_TARGET_CHANGED = {
		handler = function(...)
			ls:HandleTargetChanged(...)
		end,
	},
}

function ls:HandleEvents(event, ...)
	ls.hooks[event].handler(select(1, ...))
end

function ls:Subscribe()
	ls.events = CreateFrame("Frame")
	for k, _ in pairs(ls.hooks) do
		ls.events:RegisterEvent(k)
	end

	ls.events:SetScript("OnEvent", ls.HandleEvents)
end
