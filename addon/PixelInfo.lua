local ADDONNAME, ls = ...
local CELL_SIZE = 1.6
local METAPIXELS = 2
ls.pixelinfo = {}
ls.pixelinfo.length = 0
ls.pixelinfo.topframe = CreateFrame("Frame", "PixelInfo")
ls.pixelinfo.topframe:SetPoint("BOTTOMLEFT", 0, 0)
ls.pixelinfo.frames = {}

function ls:InitFrame(name)
	local f = CreateFrame("Frame", name, nil, "BackdropTemplate")
	f:SetHeight(CELL_SIZE)
	f:SetWidth(CELL_SIZE)
	f:SetBackdrop({
		bgFile = "Interface\\AddOns\\" .. ADDONNAME .. "\\textures\\pixel.tga",
		insets = { top = 0, left = 0, bottom = 0, right = 0 },
	})
	f:SetFrameStrata("DIALOG")
	return f
end

function ls.pixelinfo:OrderFrames()
	local framecount = ls.pixelinfo.length + METAPIXELS
	local gridwidth = math.ceil(math.sqrt(framecount))

	for i = 1, ls.pixelinfo.length + METAPIXELS do
		j = i - 1
		ls.pixelinfo.frames[i]:SetPoint(
			"BOTTOMLEFT",
			UIParent,
			"BOTTOMLEFT",
			CELL_SIZE * (j % gridwidth),
			CELL_SIZE * (math.floor(j / gridwidth))
		)
	end
end

function ls.pixelinfo:UpdatePB(pbmessage)
	local ser = pbmessage:Serialize()
	local reqpixels = math.ceil(#ser / 3)
	for i = 1, reqpixels - ls.pixelinfo.length do
		ls.pixelinfo.frames[METAPIXELS + i + ls.pixelinfo.length] = ls:InitFrame("Pixel_" .. i + ls.pixelinfo.length)
	end

	for i = 0, ls.pixelinfo.length - reqpixels - 1 do
		ls.pixelinfo.frames[ls.pixelinfo.length + METAPIXELS - i]:Hide()
	end

	ls.pixelinfo.length = reqpixels
	for i = 1, reqpixels do
		local r, g, b = string.byte(ser, 3 * (i - 1) + 1, 3 * i + 1)
		g = tonumber(g) or 0
		b = tonumber(b) or 0
		ls.pixelinfo.frames[METAPIXELS + i]:SetBackdropColor(r / 255, g / 255, b / 255)
	end
	local channel_remainder = #ser % 3
	local r = METAPIXELS + reqpixels
	-- since r can be > 256, we need to split it into two bytes using a little endian order
	local least_significant = r % 256
	local most_significant = math.floor(r / 256) % 256

	ls.pixelinfo:OrderFrames()
	ls.pixelinfo.frames[2]:SetBackdropColor(channel_remainder / 255, least_significant / 255, most_significant / 255)
end

---- create control pixels ----
ls.pixelinfo.frames[1] = ls:InitFrame("ControlPixel")
ls.pixelinfo.frames[1]:SetBackdropColor(199 / 255, 99 / 255, math.floor(2 * CELL_SIZE) / 255)
ls.pixelinfo.frames[2] = ls:InitFrame("MetaPixel")

ls.pixelinfo:OrderFrames()
